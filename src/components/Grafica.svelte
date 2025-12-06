<div class="card">
  <slot>  </slot>
</div>
<style>
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

    flex-direction: column;
    /* PRUEBA: Cambia a 'row' para poner elementos uno al lado del otro */

    justify-content: space-between;
    /* PRUEBA: center, flex-start, flex-end, space-around */

    /* align-items: center; */
    /* Descomenta la linea de arriba para centrar el contenido horizontalmente */
  }
</style>
