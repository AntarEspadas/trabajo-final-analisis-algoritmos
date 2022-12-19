from math import exp, sqrt, cos, pi
from random import randint, random, shuffle
from typing import Callable


class Individuo:

    def __init__(self, representacion: list[int]) -> None:
        """
        Crea un individuo cuyo valor está representado por el arreglo de 1s y 0s dado
        """
        self._representacion = representacion

    def individuo_aleatorio(tam: int) -> "Individuo":
        """
        Crea un individuo aleatorio
        """
        return Individuo([randint(0, 1) for _ in range(tam)])

    def valor(self) -> tuple[float, float]:
        """
        Regresa el valor representado por este individuo convirtiendo la lista 1s y 0s en
        el entero que representa en binario y dividiendo el valor entre 1000
        """
        # Divide el arreglo a la mitad, puesto que representa dos números
        mid = len(self._representacion) // 2
        # Convierte cada arreglo al número que representa en binario
        b = lista_a_int(self._representacion[mid:])
        a = lista_a_int(self._representacion[:mid])
        # Divide entre mil, ya que se quieren representar números con tres decimales
        return a / 1000, b / 1000

    def aptitud(self) -> float:
        """
        Calcula la aptitud del individuo
        """
        # Evaluar la función a optimizar en el valor que representa el individuo
        valor_funcion = f(*self.valor())
        # Como se busca minimizar el valor de la función, se usa el negativo para
        # que de esta forma la aptitud sea más grande mientras mas pequeño sea el valor de la función
        return 1000 - valor_funcion ** 2

    def mutar(self, probabilidad: float):
        """
        Itera sobre el arreglo de representación y, si el número aleatorio generado es menor
        que la probabilidad especificada, altera el valor del elemento
        """
        for i in range(len(self._representacion)):
            if random() < probabilidad:
                # (0 + 1) % 2 = 1. (1 + 1) % 2 = 0. Por lo que este enunciado invierte el bit
                self._representacion[i] = (self._representacion[i] + 1) % 2

    def copia(self) -> "Individuo":
        """
        Crea una copia del individuo
        """
        return Individuo(self._representacion)

    def cruza_un_punto(self, otro: "Individuo") -> tuple["Individuo", "Individuo"]:
        """
        Realiza una una cruzo en un punto de dos individuos
        """

        length = len(self._representacion)

        # Selecciona aleatoriamente el punto en el que se va a cortar la lista
        corte = randint(1, length - 1)

        nuevo1 = [0] * length
        nuevo2 = [0] * length

        for i in range(length):
            if i < corte:
                # Copia los elementos antes del punto de corte
                nuevo1[i] = self._representacion[i]
                nuevo2[i] = otro._representacion[i]
            else:
                # Copia los elementos del otro arreglo después del punto de corte
                nuevo1[i] = otro._representacion[i]
                nuevo2[i] = self._representacion[i]
        return Individuo(nuevo1), Individuo(nuevo2)

    def cruza_dos_puntos(self, otro: "Individuo") -> tuple["Individuo", "Individuo"]:
        """
        Realiza una una cruzo en dos puntos de dos individuos
        """

        # Sigue la misma lógica que cruza_un_punto, pero realiza la copia de un arreglo
        # cuando el índice está acotado por ambos puntos de corte

        length = len(self._representacion)

        corte1 = randint(1, length - 2)
        corte2 = randint(corte1 + 1, length - 1)

        nuevo1 = [0] * length
        nuevo2 = [0] * length

        for i in range(length):
            if corte1 <= i < corte2:
                nuevo1[i] = otro._representacion[i]
                nuevo2[i] = self._representacion[i]
            else:
                nuevo1[i] = self._representacion[i]
                nuevo2[i] = otro._representacion[i]

        return nuevo1, nuevo2


def lista_a_int(lista: list[int]) -> int:
    """Convierte una lista de 1s y 0s en el entero que representa en binario"""
    return int("".join(str(x) for x in lista), 2)


def seleccion_por_ruleta(poblacion: list[Individuo]) -> list[Individuo]:
    """
    Crea una nueva población usando selección por ruleta en la población dada
    """
    # Ordena la población para poder hacer búsqueda binaria
    poblacion.sort(key=Individuo.aptitud)

    aptitud_total = sum(individuo.aptitud() for individuo in poblacion)

    aptitudes = [0] * len(poblacion)
    aptitudes[0] = poblacion[0].aptitud() / aptitud_total

    for i in range(1, len(poblacion)):
        aptitud = poblacion[i].aptitud() / aptitud_total
        aptitudes[i] = aptitudes[i - 1] + aptitud

    nueva_poblacion = [None] * len(poblacion)

    for i in range(len(poblacion)):
        # Aquí gira la ruleta y busca el índice correspondiente usando búsqueda binaria
        j = busqueda_binaria(aptitudes, random())
        nueva_poblacion[i] = poblacion[j].copia()

    return nueva_poblacion


def busqueda_binaria(lista: list[float], x: float):
    izq = 0
    der = len(lista) - 1

    resultado = der

    while izq <= der:
        mid = (izq + der) // 2

        if lista[mid] >= x:
            resultado = mid
            der = mid - 1
        else:
            izq = mid + 1

    return resultado


def cruzar(poblacion: list[Individuo], probabilidad: float, funcion_cruza: Callable[[Individuo, Individuo], tuple[Individuo, Individuo]]):
    """
    Selecciona individuos aleatoriamente según la probabilidad dada y aplica la función de cruza especificada
    """

    individuos_seleccionados = []

    for i in range(len(poblacion)):
        # Selecciona al individuo si se cumple la probabilidad
        if probabilidad < random():
            individuos_seleccionados.append(i)

    # Mezcla aleatoriamente a los individuos seleccionados, para que las cruces sean aleatorias
    shuffle(individuos_seleccionados)

    # Si el número de individuos seleccionados no es par, descarta un individuo para que sea par
    if len(individuos_seleccionados) % 2 == 1:
        individuos_seleccionados.pop()

    # Itera de dos en dos sobre el arreglo de individuos seleccionados, cruzando los individuos y
    # reemplazándolos por sus descendientes
    for i in range(0, len(individuos_seleccionados), 2):
        j = individuos_seleccionados[i]
        k = individuos_seleccionados[i + 1]
        a, b = funcion_cruza(poblacion[j], poblacion[k])
        poblacion[j] = a
        poblacion[k] = b


def f(x1: float, x2: float) -> float:
    """La función que se quiere optimizar"""
    x = [x1, x2]
    return -20 * exp(-0.2 * sqrt(0.5 * sum(xi**2 for xi in x))) - \
        exp(0.5 * sum(cos(2 * pi * xi) for xi in x)) + 20 + exp(1)


def get_poblacion(tam_poblacion: int, tam_individuo: int) -> list[Individuo]:
    """Genera una población aleatoria del tamaño especificado"""
    return [Individuo.individuo_aleatorio(tam_individuo) for _ in range(tam_poblacion)]


def seleccion_cruza_mutacion(poblacion: list[Individuo], prob_cruza: float, prob_mutacion: float) -> list[Individuo]:
    """Realiza los pasos de selección, cruza y mutación requeridos para el algoritmo genético"""
    poblacion = seleccion_por_ruleta(poblacion)
    cruzar(poblacion, prob_cruza, Individuo.cruza_un_punto)
    for individuo in poblacion:
        individuo.mutar(prob_mutacion)
    return poblacion


def busca_tiempo_indefinido(tam_poblacion: float, prob_cruza: float, prob_mutacion: float) -> Individuo:
    """Busca mejorar la población hasta que el usuario la interrumpa"""
    poblacion = get_poblacion(tam_poblacion, 30)
    iteracion = 1
    mejor_individuo = max(poblacion, key=Individuo.aptitud)
    try:
        while True:
            if iteracion % 100 == 0:
                total = sum(i.aptitud()
                            for i in poblacion)
                promedio = total / len(poblacion)
                print(f"Mejor valor: {mejor_individuo.valor()}")
            poblacion = seleccion_cruza_mutacion(
                poblacion, prob_cruza, prob_mutacion)
            mejor = max(poblacion, key=Individuo.aptitud)
            if mejor_individuo.aptitud() < mejor.aptitud():
                mejor_individuo = mejor
            iteracion += 1

    except KeyboardInterrupt:
        pass

    return mejor_individuo


def main():
    mejor_individuo = busca_tiempo_indefinido(100, 0.25, 0.01)
    print(f"Mejor individuo: {mejor_individuo}")
    print(f"Valor: {mejor_individuo.valor()}")
    print(f"Aptitud: {mejor_individuo.aptitud()}")


if __name__ == "__main__":
    main()
