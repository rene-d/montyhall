#!/usr/bin/env python3

from argparse import ArgumentParser
from random import shuffle


def choix(portes, exclusion=[], count=1):
    """
    Retourne un nombre aléatoire entre `1` et `portes`, sauf ceux exclus.
    """
    r = list(set(range(1, portes + 1)) - set(exclusion))
    shuffle(r)
    return r[:count]


def simulation(portes, tours, ouverture):
    """
    Simulation du problème de Monty Hall.
    """
    sans_changement = 0
    avec_changement = 0

    for tour in range(tours):
        voiture = choix(portes)[0]
        joueur = choix(portes)[0]

        presentateur = choix(portes, [voiture, joueur], count=ouverture)

        # Le joueur ne change pas de porte
        if voiture == joueur:
            sans_changement += 1
        else:
            # Le joueur change de porte
            second = choix(portes, [*presentateur, joueur])
            if voiture in second:
                avec_changement += 1

    print(f"portes={portes} tours={tours} sans={sans_changement/tours:.5f} avec={avec_changement/tours:.5f}")

    p = 1 / portes
    q = p * (portes - 1) / (portes - ouverture - 1)
    print(f"portes={portes} tours={tours} sans={p:.5f} avec={q:.5f}")


def main():
    parser = ArgumentParser("Monty Hall")
    parser.add_argument("-p", "--portes", type=int, default=3, help="Nombre de portes")
    parser.add_argument("-t", "--tours", type=int, default=100_000, help="Nombre de tours")
    parser.add_argument("-o", "--ouverture", type=int, default=1, help="Nombre de portes ouvertes par le présentateur")
    args = parser.parse_args()

    if 1 + args.ouverture >= args.portes:
        print("Erreur: le nombre de portes ouvertes par le présentateur est trop grand.")
        exit(1)

    simulation(args.portes, args.tours, args.ouverture)


if __name__ == "__main__":
    main()
