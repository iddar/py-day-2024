# Invocando la Interfaz Lorem (Rust) desde Python

## Control Total desde Python

Con Floem, es posible invocar la interfaz gráfica directamente desde un script de Python, aprovechando PyO3 para establecer una comunicación fluida entre ambos lenguajes.

## Uso de Signals

Floem ofrece un sistema de reactividad basado en signals que permite actualizar dinámicamente los valores de la UI en tiempo real desde Python.

### Ejemplo

Cambiar un valor numérico o texto mostrado en la interfaz es tan simple como modificar la señal asociada desde Python.

## Versatilidad en la Interacción

Esto habilita la creación de interfaces altamente personalizables, donde los datos pueden ser manipulados directamente por la lógica del programa escrito en Python.
Esta estructura modular ofrece varias ventajas:

1. **Mejor organización**: Cada módulo tiene una responsabilidad específica.
2. **Mantenibilidad**: Es más fácil encontrar y modificar código específico.
3. **Reutilización**: Los componentes están mejor aislados y son más fáciles de reutilizar.
4. **Legibilidad**: El código está mejor organizado y es más fácil de entender.

Los módulos están organizados de la siguiente manera:
- `lib.rs`: Punto de entrada y configuración del módulo Python
- `state.rs`: Gestión del estado global
- `app.rs`: Configuración y lanzamiento de la aplicación
- `ui/mod.rs`: Componentes de UI reutilizables
- `views/mod.rs`: Vistas principales de la aplicación

Para usar esta estructura, asegúrate de crear los directorios correspondientes en tu proyecto:

```
src/
├── lib.rs
├── state.rs
├── app.rs
├── ui/
│   └── mod.rs
└── views/
    └── mod.rs
```
