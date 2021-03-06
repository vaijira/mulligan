"use strict";

function entity(character) {
    return `&#${character.charCodeAt(0).toString()};`;
}

function createHTMLElement(html) {
    var template = document.createElement("template");
    html = html.trim();
    template.innerHTML = html;
    return template.content.firstChild;
}

function swatches({
    color,
    resource,
    marginLeft = 0
}) {
    var elements = color.domain().map(value => {
        const label = value;
        return `<div class="fed-concept-item">
    <div class="fed-concept-swatch" style="background:${color(value)};"></div>
    <div class="fed-concept-label" title="${label.replace(/["&]/g, entity)}">${label}</div>
  </div>`;
    });

    var appendFedResourceItem = function (resource) {
        return function appendFedItem(item, index, arr) {
            const html_item = createHTMLElement(item);
            document.getElementById("fed-" + resource + "-items").appendChild(html_item);
        }
    };

    elements.forEach(appendFedResourceItem(resource));
}

function chart(width, height, series, color, area, xAxis, yAxis) {
    const svg = d3.create("svg").attr("viewBox", [0, 0, width, height]);

    svg.append("g")
        .selectAll("path")
        .data(series)
        .join("path")
        .attr("fill", ({ key }) => color(key))
        .attr("d", area)
        .append("title")
        .text(({key}) => key);

    svg.append("g").call(xAxis);

    svg.append("g").call(yAxis);

    return svg.node();
}

function drawResource(resource) {
    const fileName = 'tmp/' + resource + '.csv';
    const chartId = resource + '-chart';
    d3.csv(fileName)
        .then(function (data) {
            var data = Object.assign(data, { y: "Millions USD" });

            var colors = d3.scaleOrdinal(data.columns.slice(1), d3.schemePaired);
            swatches({
                color: colors,
                resource: resource,
            });

            const margin = ({ top: 20, right: 30, bottom: 30, left: 80 });

            const height = 500;
            const width = 958;

            const series = d3.stack().keys(data.columns.slice(1))(data);

            var xRange = d3.extent(data, d => d.date);
            var xDateRange = [new Date(xRange[0]), new Date(xRange[1])];

            var x = d3.scaleUtc()
                .domain(xDateRange)
                .range([margin.left, width - margin.right]);

            var y = d3.scaleLinear()
                .domain([0, d3.max(series, d => d3.max(d, d => d[1]))])
                .nice()
                .range([height - margin.bottom, margin.top]);

            var area = d3.area()
                .x(d => x(new Date(d.data.date)))
                .y0(d => y(d[0]))
                .y1(d => y(d[1]));

            var xAxis = g => g
                .attr("transform", `translate(0,${height - margin.bottom})`)
                .call(d3.axisBottom(x).ticks(width / 40).tickSizeOuter(0));

            var yAxis = g => g
                .attr("transform", `translate(${margin.left},0)`)
                .call(d3.axisLeft(y))
                .call(g => g.select(".domain").remove())
                .call(g => g.select(".tick:last-of-type text").clone()
                    .attr("x", 3)
                    .attr("text-anchor", "start")
                    .attr("font-weight", "bold")
                    .text(data.y));

            const svgChart = chart(width, height, series, colors, area, xAxis, yAxis);
            document.getElementById(chartId).appendChild(svgChart);
        })
        .catch(function (error) {
            console.error(error);
        })
}