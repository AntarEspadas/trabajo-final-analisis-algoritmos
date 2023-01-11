use bitvec::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::mem::transmute;

use std::f64::consts::{E, PI};

#[derive(Clone)]
pub struct Individuo {
    //Se puede usar un entero de 32 bits para almacenar el arreglo de 32
    //bits con el que se representa cada solución
    cromosomas: u32,
    aptitud: f64,
}

impl Individuo {
    ///Devuelve la aptitud del individuo
    pub fn aptitud(&self) -> f64 {
        self.aptitud
    }

    ///Crea un nuevo individuo con la representación dada
    pub fn new(cromosomas: u32) -> Self {
        let mut resultado = Self {
            cromosomas,
            aptitud: 0.0,
        };
        //Calcula la aptitud del individuo después de crearlo
        resultado.calcular_aptitud();
        resultado
    }

    ///Crea un individuo aleatorio
    pub fn aleatorio() -> Self {
        let mut rng = rand::thread_rng();
        //Crea un individuo con un entero de 32 bits aleatorio
        Self::new(rng.gen())
    }

    ///Devuelve el valor de la solución que representa el individuo
    pub fn valor(&self) -> (f64, f64) {
        //Convierte el entero de 32 bits en
        //una tupla de enteros de 16 bits
        let (a, b): (i16, i16) = unsafe { transmute(self.cromosomas) };
        //Divide cada uno de los valores entre 1000 para
        //obtener valores entre -32.768 y 32.768
        (a as f64 / 1000.0, b as f64 / 1000.0)
    }

    ///Aplica una mutación a los bits del individuo con la probabilidad dada
    pub fn mutar(&mut self, probabilidad: f32) {
        //Interpreta el entero de 32 bits como un arreglo
        let bits = self.cromosomas.view_bits_mut::<Msb0>();
        let mut rng = rand::thread_rng();

        let mut mutado = false;

        for mut bit in bits.iter_mut() {
            //Genera un número aleatorio entre 0 y 1. Si el número es
            //menor que la probabilidad dada, cambia el bit y pone la
            //bandera `mutado` como true, para indicar que la aptitud
            //del individuo debe volverse a calcular
            if rng.gen_range(0.0..1.0) < probabilidad {
                *bit = !*bit;
                mutado = true;
            }
        }

        //Si el individuo sufrió mutaciones, volver a calcular la aptitud
        if mutado {
            self.calcular_aptitud()
        }
    }

    ///Calcula y almacena la aptitud del individuo
    fn calcular_aptitud(&mut self) {
        //Obtiene los valores de x1 y x2 que representa este individuo
        let (x1, x2) = self.valor();
        //Multiplica la función por -1, puesto que se busca un mínimo y
        //suma 50 para evitar negativos
        self.aptitud = -f(x1, x2) + 50.0;
    }

    /// Cruza al individuo con otro individuo usando cruza en un punto
    /// para obtener un nuevo par de individuos
    pub fn cruza_un_punto(&self, otro: &Self) -> (Individuo, Individuo) {
        let mut rng = rand::thread_rng();

        // Crea las variables donde se van a almacenar
        // los cromosomas de los descendientes
        let mut descendiente1: u32 = 0;
        let mut descendiente2: u32 = 0;

        // Interpreta los enteros como arreglos de bits
        let bits_descendiente1 = descendiente1.view_bits_mut::<Msb0>();
        let bits_descendiente2 = descendiente2.view_bits_mut::<Msb0>();

        // Los enteros de los padres interpretados como arreglos de bits
        let bits_self = self.cromosomas.view_bits::<Msb0>();
        let bits_otro = otro.cromosomas.view_bits::<Msb0>();

        // Escoge un punto aleatorio para la cruza
        let punto = rng.gen_range(1..bits_self.len());

        for i in 0..bits_self.len() {
            // Copia todos los bits antes del punto a un descendiente y
            // el resto a otro descendiente
            if i < punto {
                bits_descendiente1.set(i, bits_self[i]);
                bits_descendiente2.set(i, bits_otro[i]);
            } else {
                bits_descendiente2.set(i, bits_self[i]);
                bits_descendiente1.set(i, bits_otro[i]);
            }
        }

        (Individuo::new(descendiente1), Individuo::new(descendiente2))
    }

    /// Cruza al individuo con otro individuo usando cruza en dos
    /// puntos para obtener un nuevo par de individuos
    pub fn cruza_dos_puntos(&self, otro: &Self) -> (Individuo, Individuo) {
        let mut rng = rand::thread_rng();

        let mut descendiente1: u32 = 0;
        let mut descendiente2: u32 = 0;

        let bits_descendiente1 = descendiente1.view_bits_mut::<Msb0>();
        let bits_descendiente2 = descendiente2.view_bits_mut::<Msb0>();

        let bits_self = self.cromosomas.view_bits::<Msb0>();
        let bits_otro = otro.cromosomas.view_bits::<Msb0>();

        let punto1 = rng.gen_range(1..bits_self.len() - 1);
        let punto2 = rng.gen_range(punto1 + 1..bits_self.len());

        for i in 0..bits_self.len() {
            // Similar a cruza en un punto, pero copia todos los bits
            // que están entre ambos puntos a un descendiente y el
            // resto al otro descendiente
            if punto1 <= i && i < punto2 {
                bits_descendiente1.set(i, bits_self[i]);
                bits_descendiente2.set(i, bits_otro[i]);
            } else {
                bits_descendiente2.set(i, bits_self[i]);
                bits_descendiente1.set(i, bits_otro[i]);
            }
        }

        (Individuo::new(descendiente1), Individuo::new(descendiente2))
    }
}

pub struct Poblacion {
    individuos: Vec<Individuo>,
    mejor_individuo: Option<Individuo>,
}

impl Poblacion {
    /// Crea una nueva población usando el vector de individuos proporcionado
    pub fn new(individuos: Vec<Individuo>, mejor_individuo: Option<Individuo>) -> Self {
        let mut resultado = Self {
            individuos,
            mejor_individuo,
        };

        // Encuentra el mejor individuo de la población antes de regresarla
        resultado.guardar_mejor_individuo();

        resultado
    }

    /// Devuelve una población aleatoria del tamaño dado
    pub fn aleatoria(tam_pop: usize) -> Self {
        let mut individuos = Vec::with_capacity(tam_pop);

        // Añade individuos aleatorios a la población
        for _ in 0..tam_pop {
            individuos.push(Individuo::aleatorio());
        }

        Poblacion::new(individuos, None)
    }

    /// Encuentra y almacena al mejor individuo
    fn guardar_mejor_individuo(&mut self) {
        // Encuentra al individuo con la mayor aptitud dentro de la población
        let mejor_potencial = self
            .individuos
            .iter()
            .max_by(|a, b| a.aptitud().partial_cmp(&b.aptitud()).unwrap())
            .unwrap();

        // Compara al mejor individuo de la población con el mejor global,
        // si es mejor, remplaza al mejor global
        if let Some(individuo) = &self.mejor_individuo {
            if individuo.aptitud() >= mejor_potencial.aptitud() {
                return;
            }
        }

        self.mejor_individuo = Some(mejor_potencial.clone());
    }

    /// Devuelve al mejor individuo entre esta población y todas las anteriores
    pub fn mejor_individuo(&self) -> &Individuo {
        match &self.mejor_individuo {
            Some(individuo) => individuo,
            None => panic!(),
        }
    }

    /// Devuelve la aptitud promedio de la población
    pub fn aptitud_promedio(&self) -> f64 {
        self.individuos.iter().map(Individuo::aptitud).sum::<f64>() / self.individuos.len() as f64
    }

    /// Devuelve una nueva población aplicando selección por ruleta a esta población
    pub fn seleccion_por_ruleta(&self) -> Self {
        let mut rng = rand::thread_rng();

        // La biblioteca rand de rust ya cuenta con un método para seleccionar números en una lista con un cierto peso
        let nuevos_individuos: Vec<Individuo> = self
            .individuos
            .choose_multiple_weighted(&mut rng, self.individuos.len(), Individuo::aptitud)
            .unwrap()
            .cloned()
            .collect();

        // Crea una nueva población con los individuos seleccionados, manteniendo al mejor
        // individuo que se ha encontrado hasta ahora
        Poblacion::new(nuevos_individuos, self.mejor_individuo.clone())
    }

    /// Devuelve una nueva población aplicando selección por torneo a esta población
    pub fn seleccion_por_torneo(&self) -> Self {
        let mut rng = rand::thread_rng();
        let mut nueva_poblacion = Vec::new();
        for _ in 0..self.individuos.len() {
            let ganador = self
                .individuos
                // Escoge aleatoriamente a dos individuos de la población
                .choose_multiple(&mut rng, 2)
                // Se queda con el individuo con la mayor aptitud entre esos dos
                .max_by(|a, b| a.aptitud().partial_cmp(&b.aptitud()).unwrap())
                .unwrap();
            nueva_poblacion.push(ganador.clone());
        }

        // Crea una nueva población con los individuos seleccionados, manteniendo al mejor
        // individuo que se ha encontrado hasta ahora
        Self::new(nueva_poblacion, self.mejor_individuo.clone())
    }

    /// Selecciona individuos dentro de la población, los cruza con la función proporcionada
    /// y reemplaza a los padres con sus descendientes
    pub fn cruzar(
        &mut self,
        probabilidad: f32,
        funcion_cruza: impl Fn(&Individuo, &Individuo) -> (Individuo, Individuo),
    ) {
        let mut rng = rand::thread_rng();
        // Para cada índice de la lista de individuos, genera un número aleatorio entre 0 y 1,
        // si el valor es menor a la probabilidad dada, selecciona ese índice
        let mut indices_seleccionados: Vec<usize> = (0..self.individuos.len())
            .filter(|_| rng.gen_range(0.0..1.0) < probabilidad)
            .collect();

        // Pone los índices en un orden aleatorio
        indices_seleccionados.shuffle(&mut rng);

        // Si el número de índices seleccionado no es par, quitar
        // un índice aleatorio
        if indices_seleccionados.len() % 2 != 0 {
            indices_seleccionados.pop();
        }

        // Itera sobre los índices de dos en dos, cruza a los individuos en esos
        // índices y los reemplaza por sus descendientes
        for indices in indices_seleccionados.chunks(2) {
            let i = indices[0];
            let j = indices[1];

            let (descendiente1, descendiente2) =
                funcion_cruza(&self.individuos[i], &self.individuos[j]);

            self.individuos[i] = descendiente1;
            self.individuos[j] = descendiente2
        }

        // Como la población cambió, hay que volver a encontrar al mejor individuo
        self.guardar_mejor_individuo();
    }

    /// Muta a los individuos de la población con la probabilidad dada
    pub fn mutar(&mut self, probabilidad: f32) {
        for individuo in self.individuos.iter_mut() {
            individuo.mutar(probabilidad);
        }

        self.guardar_mejor_individuo();
    }
}

/// Implementación de la función que se busca optimizar
pub fn f(x1: f64, x2: f64) -> f64 {
    -20.0 * E.powf(-0.2 * (0.5 * (x1.powi(2) + x2.powi(2))).sqrt())
        - E.powf(0.5 * ((2.0 * PI * x1).cos() + (2.0 * PI * x2).cos()))
        + 20.0
        + E
}

/// Crea una población aleatoria y realiza múltiples iteraciones
/// del algoritmo genético hasta que transcurra un cierto número de
/// iteraciones sin ninguna mejora, tras lo cual devuelve a la población
/// final, al mejor individuo y la cantidad de iteraciones que llevó encontrarlo
pub fn encuentra_mejor_individuo(
    tam_pop: usize,
    max_iteraciones_sin_mejora: usize,
    probabilidad_cruza: f32,
    probabilidad_mutacion: f32,
    funcion_cruza: impl Fn(&Individuo, &Individuo) -> (Individuo, Individuo),
    funcion_seleccion: impl Fn(&Poblacion) -> Poblacion,
) -> (Poblacion, Individuo, usize) {
    let mut poblacion = Poblacion::aleatoria(tam_pop);

    let mut iteraciones_sin_mejora = 0;

    let mut mejor_individuo = poblacion.mejor_individuo().clone();

    let mut iteraciones = 0;

    // Inicia un ciclo infinito
    for i in 0.. {
        iteraciones = i;
        iteraciones_sin_mejora += 1;
        // Compara al mejor individuo de la población actual con el mejor
        // que se ha visto hasta ahora para ver si ha habido mejoras
        if mejor_individuo.aptitud() < poblacion.mejor_individuo().aptitud() {
            mejor_individuo = poblacion.mejor_individuo().clone();
            iteraciones_sin_mejora = 0;
        }
        if iteraciones_sin_mejora == max_iteraciones_sin_mejora {
            break;
        }

        poblacion = funcion_seleccion(&poblacion);
        poblacion.cruzar(probabilidad_cruza, &funcion_cruza);
        poblacion.mutar(probabilidad_mutacion);
    }

    (
        poblacion,
        mejor_individuo,
        iteraciones - iteraciones_sin_mejora + 1,
    )
}
