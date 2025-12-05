<div class="card text-center">
  <slot>  </slot>
</div>
<style>
  .card {
    /* --- 1. FONDO Y EFECTOS --- */
    background: #ffffff;
    /* PRUEBA ESTOS FONDOS: */
    /*background: #1a1a1a; /*(Modo oscuro) */
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); (Gradiente)
    /*background: rgba(255, 255, 255, 0.5);/* */
    border-radius: 1px;
    border: 5px solid #C01313;
    box-shadow: 0 4px 20px rgba(0,0,0,0.08);
    /* PRUEBA: Cambia el color de la sombra a azul: rgba(0, 100, 255, 0.2) */

    padding: 1.5rem;
    height: 100%;

    /* --- 2. DISPOSICIÓN (FLEXBOX) --- */
    display: flex;

    flex-direction: column;
    /* PRUEBA: Cambia a 'row' para poner elementos uno al lado del otro */

    justify-content: space-between;
    /* PRUEBA: center, flex-start, flex-end, space-around */

    /* align-items: center; */
    /* Descomenta la linea de arriba para centrar el contenido horizontalmente */
  }
</style>
