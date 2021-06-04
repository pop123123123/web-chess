<template>
  <button><div><slot></slot></div></button>
</template>

<style lang="scss" scoped>
@use '../scss/theme';

@mixin buttonColoring($name) {
  & {
    background: nth(map-get(theme.$colors, $name), 1);

    > div {
      background: nth(map-get(theme.$colors, $name), 2);
      color: nth(map-get(theme.$colors, $name), 3);
    }

    &:active {
      background: nth(map-get(theme.$colors, $name), 2);

      > div {
        background: nth(map-get(theme.$colors, $name), 1);

      }
    }
  }
}

button {
  padding: 5px;
  border-radius: 8px;
  border: 3px solid #00000066;
  @include buttonColoring('default');

  > div {
    border-radius: 4px;
    padding: 5px;
    text-align: center;
    font-size: 0.9em;
  }

  &:hover {
    filter: brightness(1.1);
  }
}

@each $class, $colors in theme.$colors {
  button.#{$class} {
    @include buttonColoring($class);
  }
}
</style>
