#!/usr/bin/env python3

from random import randrange


PORTES = 3
TOURS = 100_000


def choix(*autres):
    """
    Retourne un nombre aléatoire entre `1` et `PORTES`, sauf ceux passés en paramètre.
    """
    while True:
        r = randrange(1, PORTES + 1)
        if r not in autres:
            return r


sans_changement = 0
avec_changement = 0

for tour in range(TOURS):
    voiture = choix()
    joueur = choix()

    if joueur == voiture:
        # Le joueur choisit la porte à voiture
        presentateur = choix(voiture)
    else:
        # Le joueur choisit une porte à chèvre
        presentateur = choix(voiture, joueur)

    # Le joueur ne change pas de porte
    if joueur == voiture:
        sans_changement += 1

    # Le joueur change de porte
    second = choix(presentateur, joueur)
    if second == voiture:
        avec_changement += 1


print(f"portes={PORTES} tours={TOURS} sans={sans_changement/TOURS:.5f} avec={avec_changement/TOURS:.5f}")
print(f"portes={PORTES} tours={TOURS} sans={1/PORTES:.5f} avec={1/PORTES*(1+1/(PORTES-2)):.5f}")
