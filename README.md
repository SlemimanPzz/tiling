# Adoquinamiento

Adoquina una area de $m \times m$ donde $m = 2^k$ donde $k \in \mathbb{N}$ con adoquines en forma de L.

## Ejecución

Este programa esta hecho en [Rust](https://www.rust-lang.org).
Por lo que es necesario tenerlo instalado, [aqui link](https://www.rust-lang.org/tools/install), 
o en general en computadoras corriendo 
macOS Linux, u otros sistemoas operativos similares linix deberia ser 
suficiente con

### Instalar

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Ahora para para la manera recomendada de ejecucion es

```bash
cargo run [k]
```

o tambien

```bash
cargo run [k] [x] [y]
```

donde $x$ y $y$ indican donde se colocara el cuadrado especial.

#### Alternativas

Tambien se puede unicamente construir el ejecutable con 

```bash
cargo build
```

y ejecutarse con 

```bash
./target/debug/tiling [k]
````

o bien si deseas colocar el cuadro especial

```bash
./target/debug/tiling [k] [x] [y]
```

Aunque existen mas, estas seran las cubiertas aqui.
