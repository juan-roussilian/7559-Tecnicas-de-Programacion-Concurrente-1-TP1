# Tecnicas de programacion Concurrente I - 75.59
## *Informe TP1*
### Alumno: Juan Cruz Roussilian 
### Pádron: 14269
---
<br>

## **Ejecucion del programa**
El programa de ejecuta mediante el comando
<pre>
cargo run
</pre>
y analogamente los tests automatizados con
<pre>
cargo test
</pre>

##  **Diseño**
<br>

### Modelado

El modelo del dominio consta de 3 clases/structs principales. `Dispensador`, `Contenedor` y `ContenedorRecarga`. Estos ultimos implementan una interfaz 
`ContenedorCafetera` la cual implica que pueden responder a los mensajes `obtener_contenido` y `nivel`

La manera de interactuar de estas clases es tal que un dispensador contiene 4 contenedores fijos (cafe molido, agua caliente, espuma de leche y cacao) a los cuales les pide su contenido de manera thread safe para preparar las bebidas, y estos a su vez pueden o no tener un contenedor de recarga en caso de quedarse sin suficientes recursos para el pedido.

Los recursos se extraen de los contenedores consumiendo ademas del recurso, un tiempo dependiendo de la cantidad tomada.

El factor de conversion de tiempo que requiere la extraccion de una unidad de un ingrediente, junto a otras constantes como los tamaños de cada contenedor, porcentaje de alerta de bajo nivel de  un contenedor, tiempo entre estadisticas y el archivo del cual se tomaran las instrucciones son configurables mediante el archivo `constants.rs`

Ademas tambien existe la clase `ConsoleLogger` la cual se guarda referencia a los contenedores y consumos de los mismos para despues de un tiempo configurable por el usuario mostrar las estadisticas requeridas por la consigna.
<br><br>


### Concurrencia

La concurrencia se diseñó e implementó de una manera directa y relativamente sencilla siendo que se crea un hilo para la cantidad de dispensadores que elija el usuario, los cuales inmediatamente comienzan a preparar bebidas en base a los pedidos. Ademas se crea un hilo propio para el console logger y este inmediatamente se dedica a imprimir las estadisticas por intervalos. Para los objetos que componen el estado mutable compartido, osea contenedores, lista de pedidos, y contadores requeridos por las estadisticas, se decidió implementar la exclusion mutua mediante **locks de lectura y escritura** unicamente.

Es decir, todos los componentes que conforman la seccion critica estan dentro de un objeto `RwLock`. Osea que por cada hilo de dispensador, este tomara el lock de instrucciones, tomará una instrucción, y en base a la "receta" de la misma, tomara los locks (para escritura) de los contenedores correspondientes para obtener los ingredientes. El hilo del logger tomara los locks unicamente para lectura.

Se tomó esta decisión por la naturaleza del problema y la simplicidad, ya que todos los recursos eran únicos en su clase (no hay mas de un contenedor de cada tipo) y de implementarse contrariamente la exclusión mutua con un semáforo, estos terminarian siendo semáforos binarios y los cuales son equivalentes a los locks. 

---

## **Puntos a mejorar**

Se detectaron cuestiones a implementar para mejorar el desempeño de la aplicación pero que por falta de tiempo no fueron programadas. Se detallan a continuación las mas importantes:
1. Utilizar un semáforo para proceso de producción y consumo de pedidos en lugar de cargarlas todas a memoria de una sola vez y que luego el programa tome el lock de pedidos para cuando lo necesitase. Para esto seria necesario tener un hilo que se encargue de producir pedidos, y es dificil de implementar para que no se realice en una sola instruccion ya que se utilizo el parseo de Serde. Podría partirse el string de json de pedidos pero esto agregaría complejidad e incluso podría empeorar la performance.
2. Tener una manera mas amigable para el usuario para configurar las constantes. Esto podría mejorarse con interacíon con la aplicación mediante entrada salida o algun formato de archivo mas común/génerico que archivos de rust.
3. Utilizar condvars para avisar el bajo nivel de un contenedor.
4. Mayor cantidad de tests automatizados. Tanto tests concurrentes como tests para la clase dispensador


