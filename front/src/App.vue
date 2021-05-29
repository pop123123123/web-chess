<template>
  <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/game">Game</router-link> |
    <router-link to="/about">About</router-link>
  </div>
  <router-view/>
</template>

<style lang="scss">
body {
  display: flex;
  background: darkolivegreen;
  margin: 0;
  min-height: 100vh;
}

#app {
  display: flex;
  flex-direction: column;
  flex: 1;
  margin: 0;
  color: wheat;
  font-family: sans-serif;
}

#nav {
  background: wheat;
  color: black;
  padding: 1em;

  a {
    text-decoration: none;
  }
}

a {
  color: inherit;
}

.view {
  padding: 1em;
}
</style>
