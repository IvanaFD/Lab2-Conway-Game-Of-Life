# Lab 2: Conway's Game of Life

Implementacion del juego de la vida de Conway en Rust usando `raylib-rs`. El juego se dibuja sobre un framebuffer (rejilla de 100x75) que se escala a una ventana de 800x600, pintando pixel por pixel con `set_pixel` segun el estado vivo/muerto de cada celda.

## Reglas implementadas

- Una celula viva con menos de 2 vecinos vivos muere (subpoblacion).
- Una celula viva con 2 o 3 vecinos vivos sobrevive.
- Una celula viva con mas de 3 vecinos vivos muere (sobrepoblacion).
- Una celula muerta con exactamente 3 vecinos vivos nace (reproduccion).

## Organismos del patron inicial

**Still lifes:** block, beehive, loaf, boat, tub

**Oscillators:** blinker, toad, beacon

**Spaceships:** glider, light-weight spaceship (LWSS)

Cada organismo esta implementado como su propia funcion en `src/patterns.rs`, y se siembran varias instancias de cada uno repartidas por toda la rejilla en `src/main.rs`.

## Como correrlo

```bash
cargo run
```

## Resultado

![Conway's Game of Life corriendo](lab2.gif)
