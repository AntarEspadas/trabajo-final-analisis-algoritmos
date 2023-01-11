use std::time::Instant;

use rayon::prelude::*;
use trabajo_final::{encuentra_mejor_individuo, Individuo, Poblacion};

fn main() {
    let mut datos: Vec<(String, f64)> = (1..=20)
        // Ejecuta de forma paralela múltiples corridas del algoritmo
        .into_par_iter()
        .map(|iteracion| {
            let inicio = Instant::now();

            // Ejecuta el algoritmo genético con los parámetros escogidos
            let (poblacion, mejor_individuo, iteraciones) = encuentra_mejor_individuo(
                50,
                10_000,
                0.1,
                0.01,
                Individuo::cruza_un_punto,
                Poblacion::seleccion_por_torneo,
            );

            // Mide el tiempo que tardó la ejecución
            let duracion = inicio.elapsed();

            // Redondea los valores para que sean más fáciles de visualizar
            let mut duracion = duracion.as_secs_f64();
            duracion *= 100.0;
            duracion = duracion.round();
            duracion /= 100.0;

            let mut aptitud_promedio = poblacion.aptitud_promedio();
            aptitud_promedio *= 1000.0;
            aptitud_promedio = aptitud_promedio.round();
            aptitud_promedio /= 1000.0;

            let mut aptitud = mejor_individuo.aptitud();
            aptitud *= 1000.0;
            aptitud = aptitud.round();
            aptitud /= 1000.0;

            // Pone los datos en cadenas separadas por espacios
            // para que puedan ser importados en herramientas como Matlab, Python, etc.
            (
                format!(
                    "{} {} {} {} {} \"{:?}\"",
                    iteracion,
                    aptitud_promedio,
                    aptitud,
                    iteraciones,
                    duracion,
                    mejor_individuo.valor(),
                ),
                mejor_individuo.aptitud(),
            )
        })
        .collect();

    // Ordena los resultados de mayor a menor valor de aptitud
    datos.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let datos: Vec<String> = datos.into_iter().map(|a| a.0).collect();

    // Imprime los valores la output estándar, el cual puede ser redirigido a un archivo para guardar los datos
    println!("#no_ejecucion aptitud_promedio_población aptitud_mejor_individuo iteraciones duración valor_mejor_individuo");
    println!("{}", datos.join("\n"));
}
