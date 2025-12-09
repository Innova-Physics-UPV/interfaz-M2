<div class="card">
  <slot>  </slot>
</div>

<script lang="ts">
    import { scaleLinear } from 'd3-scale';
    import { line, curveMonotoneX, curveStep, curveBasis } from 'd3-shape';
    import { min, max } from 'd3-array';
    import { datos3, etiquetas3 } from '../lib/stores.svelte.js';
    import BaseCard from './BaseCard.svelte';

    // --- 1. CONFIGURACIÓN ---
    // Dimensiones del gráfico
    let containerWidth = $state(200); // Ancho reactivo
    let height = 140;
    let margin = { top: 10, right: 40, bottom: 20, left: 40 };

    // --- 2. LÓGICA (A REALIZAR) ---

    // TODO 1: Crear Escala X ($derived)
    // Debe mapear el índice del array (0, 1, 2...) a píxeles horizontales
	const escalaX= $derived(scaleLinear()
		.domain([0, datos3.length-1])
		.range([margin.left, containerWidth-margin.right]));

    // TODO 2: Crear Escala Y ($derived)
    // Debe mapear el valor monetario (min a max) a píxeles verticales
	const escalaY= $derived(scaleLinear()
		.domain([min(datos3), max(datos3)])
		.range([height-margin.bottom, margin.top]));

    // TODO 3: Generador de Línea ($derived)
    // Debe usar d3.line() con las escalas anteriores para crear el "path"
	const pathString = $derived(line().x((dato, indice) => escalaX(indice)).y((dato) => escalaY(dato)).curve(curveMonotoneX)(datos3));


//const pathString = $derived(generador(datos));

</script>

<BaseCard>
    <div class="chart-container" bind:clientWidth={containerWidth}>
        <svg width={containerWidth} {height}>
        <!-- Capa 1: Eje Y y Rejilla -->
        {#each escalaY.ticks(4) as tick}
            <g transform="translate(0, {escalaY(tick)})">
            <line
                x1={margin.left}
                x2={containerWidth - margin.right}
                stroke="#eee"
                stroke-dasharray="4"
            />
            <text
                x={margin.left - 6}
                text-anchor="end"
                fill="#eee"
                dy="0.32em"
            >
                {tick}
            </text>
            </g>
        {/each}

        <!-- Capa 2: Eje X (Etiquetas3) -->
        {#each datos3 as d, i}
            <g transform="translate({escalaX(i)}, {height - margin.bottom})">
            <line y2="6" stroke="#eee" />
            <text
                y="20"
                text-anchor="middle"
                fill="#eee"
            >
                {etiquetas3[i % etiquetas3.length]}
            </text>
            </g>
        {/each}

        <!-- Capa 3: Línea de Datos -->
        <path
            d="{pathString}"
            fill="none"
            stroke="#00ff99"
            stroke-width="3"
            stroke-linecap="round"
        />

        <!-- Capa 4: Puntos Interactivos -->
        {#each datos3 as d, i}
            <circle
            cx={escalaX(i)}
            cy={escalaY(d)}
            r="4"
            fill="white"
            stroke="#00ff99"
            />
        {/each}
        </svg>

    </div>
</BaseCard>

<style>
    .chart-container {
        width: 100%;
        font-family: sans-serif;
        .card {
    /* --- 1. FONDO Y EFECTOS --- */
    background: #7E7EA0;
    /* PRUEBA ESTOS FONDOS: */
    /*background: #1a1a1a; /*(Modo oscuro) */
    /*background: rgba(255, 255, 255, 0.5);/* */
    border-radius: 1px;
    border: 2px solid #78787C;
    box-shadow: 0 4px 20px rgba(0,0,0,0.08);
    /* PRUEBA: Cambia el color de la sombra a azul: rgba(0, 100, 255, 0.2) */
    padding: 1.5rem;
    height: 100%;

    /* --- 2. DISPOSICIÓN (FLEXBOX) --- */
    display: flex;

    flex-direction: row;
    /* PRUEBA: Cambia a 'row' para poner elementos uno al lado del otro */

    justify-content: space-between;
    /* PRUEBA: center, flex-start, flex-end, space-around */

    /* align-items: center; */
    /* Descomenta la linea de arriba para centrar el contenido horizontalmente */
  }
    }
</style>
