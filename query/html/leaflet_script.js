let map = L.map('map', {
    maxBounds: [
        [47.1, 5.7], // Southwest coordinates
        [55.2, 16.9] // Northeast coordinates
    ],
}).setView([51.15, 11.3], 6);

L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
    maxZoom: 18,
    minZoom: 6,
    id: 'mapbox.streets',
}).addTo(map);
map.on('click', onMapClick);

let startPoint;
let startMarker;
let endPoint;
let endMarker;
let tmpMarker;
let last_path;
let xhr = new XMLHttpRequest();
let metrics;

getMetrics();

function onMapClick(e) {
    if (tmpMarker) {
        map.removeLayer(tmpMarker);
    }
    tmpMarker = L.marker(e.latlng).addTo(map);
    tmpMarker.setLatLng(e.latlng);
    tmpMarker.bindPopup("<button class='set-point set-start' onclick='setStart()''>Set Start</button><button class='set-point set-end' onclick='setEnd()''>Set End</button>").openPopup();
}

function getMetrics() {
    let xhr = new XMLHttpRequest();
    xhr.open("GET", window.location.href + "metrics", true);
    xhr.setRequestHeader("Content-type", "application/json;charset=UTF-8");

    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            metrics = JSON.parse(xhr.responseText);

            setSlider(metrics);
        } else if (xhr.readyState === 4) {
            show_invalid_request();
        }
    };
    xhr.send();
}

function setSlider(metrics) {
    let slide_container = "";
    metrics.forEach(function (item, index) {
        // console.log(item, index);
        slide_container += `
                            <label for="slider-${index}">${item}</label>
                            <input type="range" min="0" max="1" value="0.25" class="slider" step="0.01" onchange="query()" id="slider-${index}">
                            `;
    });
    document.getElementById("slidercontainer").innerHTML = slide_container;
}

function setStart() {
    let coords = tmpMarker.getLatLng();
    let lat = Math.round(coords.lat * 1000) / 1000;
    let lng = Math.round(coords.lng * 1000) / 1000;
    if (startMarker) {
        map.removeLayer(startMarker);
    }
    startPoint = tmpMarker.getLatLng();
    startMarker = L.marker(coords, {
        icon: greenIcon
    }).addTo(map);
    map.removeLayer(tmpMarker);
    if (typeof last_path === 'object') {
        map.removeLayer(last_path);
    }
    query();
}

function setEnd() {
    let coords = tmpMarker.getLatLng();
    let lat = Math.round(coords.lat * 1000) / 1000;
    let lng = Math.round(coords.lng * 1000) / 1000;
    if (endMarker) {
        map.removeLayer(endMarker);
    }
    endPoint = tmpMarker.getLatLng();
    endMarker = L.marker(coords, {
        icon: redIcon
    }).addTo(map);
    map.removeLayer(tmpMarker);
    if (typeof last_path === 'object') {
        map.removeLayer(last_path);
    }
    query();
}

function get_alpha_vector() {
    let alpa_vector = [];
    for (let i = 0; i < metrics.length; i++) {
        alpa_vector.push(parseFloat(document.getElementById("slider-" + i).value));
    }
    sum_alphas = alpa_vector.reduce((a, b) => a + b, 0);
    if (sum_alphas === 0){
        alpa_vector = alpa_vector.fill(1.0 / metrics.length);
    } else {
        alpa_vector = alpa_vector.map(function(x) { return x / sum_alphas; });
    }
    // console.log("alpa_vector", alpa_vector);
    return alpa_vector;
}

function query() {
    hide_result();
    hide_invalid_request();
    hide_no_path_found();
    hide_select_start_and_end();

    alpha_vector = get_alpha_vector();

    if (typeof last_path === 'object') {
        map.removeLayer(last_path);
    }

    if (typeof startPoint === 'undefined' || typeof endPoint === 'undefined') {
        show_select_start_and_end();
        return;
    }

    let xhr = new XMLHttpRequest();
    xhr.open("POST", window.location.href + "dijkstra", true);
    xhr.setRequestHeader("Content-type", "application/json;charset=UTF-8");

    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            let json = JSON.parse(xhr.responseText);
            if (json.path != "") {
                printPath(json);
                show_result(json.features[0].properties.cost);
            } else {
                show_no_path_found();
            }
        } else if (xhr.readyState === 4) {
            show_invalid_request();
        }
    };

    const body = {
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "geometry": {
                    "type": "Point",
                    "coordinates": [
                        startPoint.lng,
                        startPoint.lat
                    ]
                },
                "properties": {
                    "alpha": alpha_vector,
                },
            },
            {
                "type": "Feature",
                "geometry": {
                    "type": "Point",
                    "coordinates": [
                        endPoint.lng,
                        endPoint.lat
                    ]
                },
                "properties": {
                    "alpha": alpha_vector,
                },
            }
        ]
    };
    let data = JSON.stringify(body);
    // console.log("request: " + data);
    xhr.send(data);
}

function printPath(path) {
    // console.log(path);
    last_path = L.geoJSON(path)
    map.addLayer(last_path);
}


function show_invalid_request() {
    document.getElementById("invalid-request").style.display = "block";
}

function hide_invalid_request() {
    let x = document.getElementById("invalid-request");
    if (x.style.display === "block") {
        x.style.display = "none";
    }
}

function show_no_path_found() {
    document.getElementById("no-path-found").style.display = "block";
}

function hide_no_path_found() {
    let x = document.getElementById("no-path-found");
    if (x.style.display === "block") {
        x.style.display = "none";
    }
}

function show_select_start_and_end() {
    document.getElementById("select-start-and-end").style.display = "block";
}

function hide_select_start_and_end() {
    let x = document.getElementById("select-start-and-end");
    if (x.style.display === "block") {
        x.style.display = "none";
    }
}

function show_result(costs) {
    let tmp = document.getElementById("result")
    tmp.innerHTML = "costs: " + costs;
    tmp.style.display = "block";
}

function hide_result() {
    let x = document.getElementById("result");
    if (x.style.display === "block") {
        x.style.display = "none";
    }
}

const greenIcon = new L.Icon({
    iconUrl: 'img/marker-green.png',
    shadowUrl: 'img/marker-shadow.png',
    iconSize: [25, 41],
    iconAnchor: [12, 41],
    popupAnchor: [1, -34],
    shadowSize: [41, 41]
});
const redIcon = new L.Icon({
    iconUrl: 'img/marker-red.png',
    shadowUrl: 'img/marker-shadow.png',
    iconSize: [25, 41],
    iconAnchor: [12, 41],
    popupAnchor: [1, -34],
    shadowSize: [41, 41]
});
