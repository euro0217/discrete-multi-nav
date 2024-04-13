chart = document.getElementById('chart');

selector = document.getElementById('selector');
selector.onchange = (e) => {
    const file = e.target.files[0];
    if (!file) { return }

    const reader = new FileReader();
    reader.onload = e => {
        const text = e.target.result;
        if (!text) return;

        const loaddata = JSON.parse(text);
        const tmax = loaddata.seats[0].agent.length;

        const agents = Object.keys(loaddata.agents);

        const data = [
            ...getSeats(loaddata, agents, 0),
            ...getAgents(loaddata, agents, 0),
        ];

        const frames = [...Array(tmax).keys()]
            .map(t => ({
                name: t,
                data: [
                    ...getSeats(loaddata, agents, t),
                    ...getAgents(loaddata, agents, t),
                ]
            }))

        const sliderSteps = [];
        for (i = 0; i < tmax; i++) {
            sliderSteps.push({
                method: 'animate',
                label: i,
                args: [[i], {
                    mode: 'immediate',
                    transition: {duration: 0},
                    frame: {duration: 300, redraw: false},
                }]
            });
        }

        const layout = {
            updatemenus,
            yaxis: { scaleanchor: 'x'},
            sliders: [{
                pad: {l: 130, t: 55},
                currentvalue: { visible: true, prefix: 't = ', xanchor: 'right', font: {size: 15}},
                steps: sliderSteps
            }]
        };

        Plotly.newPlot(
            chart,
            { data, layout, frames },
            { margin: { t: 0 } }
        );
    }
    reader.readAsText(file)

}

const updatemenus = [{
    x: 0,
    y: 0,
    yanchor: 'top',
    xanchor: 'left',
    showactive: false,
    direction: 'left',
    type: 'buttons',
    pad: {t: 87, r: 10},
    buttons: [{
        method: 'animate',
        args: [null, {
            mode: 'immediate',
            fromcurrent: true,
            transition: {duration: 0},
            frame: {duration: 500, redraw: false}
        }],
        label: 'Play'
    }, {
        method: 'animate',
        args: [[null], {
            mode: 'immediate',
            transition: {duration: 0},
            frame: {duration: 0, redraw: false}
        }],
        label: 'Pause'
    }],
}];

const empty = i => ({
    x: [null],
    y: [null],
    fill: 'toself',
    fillcolor: i !== null  ? cyclic_color(i, 400) : '#d0d0d0',
    mode: 'none',
    name: i !== null ? `agent ${i}` : 'empty',
    opacity: 0.4,
})

const getSeats = (data, agents, t) => {
    const map = data.seats.reduce((m, s) => {
        const i = s.agent[t];
        if (!m[i]) {
            m[i] = empty(i)
        }
        const [x0, y0, x1, y1] = [s.x - 0.5, s.y - 0.5, s.x + 0.5, s.y + 0.5];
        m[i].x.push(x0, x0, x1, x1, x0, null)
        m[i].y.push(y0, y1, y1, y0, y0, null)
        return m
    }, {})

    return [map['null'], ...agents.map(i => map[i] || empty(i))]
}

const getAgents = (data, agents, t) => 
    agents.map(i => {
        const a = data.agents[i][t];
        const shape = a.shape;
        return {
            x: shape.map(p => p !== null ? p[0] : null),
            y: shape.map(p => p !== null ? p[1] : null),
            fill: 'toself',
            fillcolor: cyclic_color(i, 200),
            fillpattern: { shape: a.state === 'n' ? 'x' : '' },
            line: {
                color: cyclic_color(i, 500),
                dash: a.state === 'n' ? 'dot' : 'solid',
            },
            mode: 'lines',
            name: `agent ${i}`,
            opacity: 0.8,
        }
    });
