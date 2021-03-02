<style>
  #menu {
    width: 30%;
  }

  #menu button:hover {
    background-color: plum;
  }

  #menu button {
    display: block;
    background-color: tan;
    text-align: center;
    margin-bottom: 2px;
    width: 100%;
    font-size: 1.5em;
    border-radius: 0.5em;
    border: ridge;
  }
</style>

<script lang="ts">
  let menuVis = 'hidden';

  let expand = '( + )';

  let matVis = 'hidden';

  function menuClickHandler() {
    if (menuVis === 'hidden') {
      menuVis = 'visible';
      expand = '( - )';
    } else {
      menuVis = 'hidden';
      expand = '( + )';
    }
  }

  function matrixTextHandler() {
    if (matVis === 'hidden') {
      matVis = 'visible';
    } else {
      matVis = 'hidden';
    }
  }
</script>

<!-- TODO: Make Menu foldable  -->
<div id="menu">
  <button id="pg" on:click={menuClickHandler}>{expand} Menu</button>
  <button id="pg1" on:click={matrixTextHandler} style="visibility: {menuVis};">Matrix Multiplication</button>
  <button id="pg2" style="visibility: {menuVis};">Rotation Matrices</button>
  <button id="pg3" style="visibility: {menuVis};">Quaternions</button>
  <button id="pg4" style="visibility: {menuVis};">Misc</button>
</div>

<div id="matrix1">
  <form action="/" id="matform" style="visibility: {matVis};">
    <input type="textarea">
  </form>
</div>
