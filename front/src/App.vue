<template>
  <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link>
  </div>
  <router-view/>
</template>

<style lang="scss">
@use './scss/theme';

body {
  display: flex;
  background: theme.$background-main;
  margin: 0;
  min-height: 100vh;
}

#app {
  display: flex;
  flex-direction: column;
  flex: 1;
  margin: 0;
  color: theme.$color-text;
  font-family: sans-serif;
}

#nav {
  background: theme.$background-header;
  color: theme.$color-text-dark;
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
