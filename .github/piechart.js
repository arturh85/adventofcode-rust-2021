const fs = require("fs");
const prefix = 'https://quickchart.io/chart?c=';

let chart = {
    "type": "outlabeledPie",
    "data": {
        "labels": [],
        "datasets": [{
            "backgroundColor": [],
            "data": []
        }]
    },
    "options": {
        "plugins": {
            "legend": false,
            "outlabels": {
                "text": "%l %p",
                "color": "white",
                "stretch": 35,
                "font": {
                    "resizable": true,
                    "minSize": 12,
                    "maxSize": 18
                }
            }
        }
    }
};

const buffer = fs.readFileSync("times.md", 'utf-8');
const regex = /\#\# Day (\d+) Part (1|2)/;
const regex2 = /- (\w+): ([\d.]+)([µ\w]+)/;
const colors = [
    '#c42cb2',
    '#065535',
    '#000000',
    '#133337',
    '#9d1e32',
    '#008080',
    '#ff0000',
    '#ffa500',
    '#ff7373',
    '#40e0d0',
    '#0000ff',
    '#b0e0e6',
    '#d3ffce',
    '#666666',
    '#bada55',
    '#003366',
    '#fa8072',
    '#ffb6c1',
    '#9b9a9a',
    '#800000',
    '#800080',
    '#c39797',
    '#f08080',
    '#00ff00',
    '#cccccc',
    '#20b2aa',
    '#333333',
    '#ffc3a0',
    '#66cdaa',
    '#ff6666',
    '#ff00ff',
    '#ff7f50',
    '#468499',
    '#008000',
    '#cbbeb5',
    '#afeeee',
    '#f6546a',
    '#00ced1',
    '#b6fcd5',
    '#660066',
    '#b4eeb4',
    '#daa520',
    '#0e2f44',
    '#990000',
    '#696969',
    '#808080',
    '#6897bb',
    '#8b0000',
    '#088da5',
    '#000080',
    '#101010',
    '#0a75ad',
    '#8a2be2',
    '#2acaea',
    '#81d8d0',
    '#ff4040',
    '#66cccc',
    '#420420',
    '#00ff7f',
    '#794044',
    '#ff1493',
    '#a0db8e',
    '#999999',
    '#3399ff',
    '#191970',
];

let day = -1;
let part = -1;
let time_in_micro_seconds = -1;
let total_time_in_micro_seconds = 0;

const lines = buffer.split(/\r?\n/);

let colorIdx = 0;
for (line of lines) {
    const matches = line.match(regex);
    if (matches) {
        if (day !== -1) {
            chart.data.labels.push(`Day ${day}.${part}`)
            chart.data.datasets[0].backgroundColor.push(colors[colorIdx++])
            chart.data.datasets[0].data.push(time_in_micro_seconds)
        }
        day = matches[1];
        part = matches[2];
        total_time_in_micro_seconds += time_in_micro_seconds;
        time_in_micro_seconds = 0;
    }
    const matches2 = line.match(regex2);
    if (matches2) {
        const amount = matches2[2];
        const unit = matches2[3];
        switch(unit) {
            case "ns":
                time_in_micro_seconds += 1; break;
            case "µs":
                time_in_micro_seconds += Math.ceil(1 * amount); break;
            case "ms":
                time_in_micro_seconds += Math.ceil(1000 * amount); break;
            case "s":
                time_in_micro_seconds += Math.ceil(1000000 * amount); break;
            default:
                console.error(`invalid unit: ${unit}`);
                process.exit(1)
        }
    }
}

if (day !== -1) {
    chart.data.labels.push(`Day ${day}.${part}`)
    chart.data.datasets[0].backgroundColor.push(colors[colorIdx++])
    chart.data.datasets[0].data.push(time_in_micro_seconds)
    total_time_in_micro_seconds += time_in_micro_seconds;
}

lines[4] = '## Distribution of ' + (total_time_in_micro_seconds / 1000.0).toFixed(2) + " ms";
lines[5] = '![Pie Chart](' + prefix + encodeURIComponent(JSON.stringify(chart)) + ')';
fs.writeFileSync("times.md", lines.join('\n'));
