# Recette Windows, clavier et NVDA

Statut : protocole à exécuter. Il ne constitue pas une validation NVDA ou DPI.

Consigner la version de Windows, de PDFnorris et de NVDA, la résolution, le facteur
Windows, la taille d’interface choisie et le périphérique utilisé. Garder le
rapport détaillé et les captures de documents personnels hors du dépôt.

## Matrice

Exécuter les mêmes parcours à 100 %, 125 %, 150 % et 200 % Windows, avec chacune
des tailles Normale, Grande et Très grande. Utiliser une fenêtre maximisée, une
fenêtre réduite et deux écrans de facteurs différents. Refaire le déplacement
entre écrans dans les deux sens. Les simulations de petites surfaces logiques
ne remplacent pas cette matrice sur le système réel.

## Clavier et lecteur d’écran

1. Lancer NVDA puis PDFnorris sans document. Ouvrir un PDF synthétique avec Ctrl+O.
2. Parcourir les commandes avec Tab et Maj+Tab. Consigner tout contrôle inaccessible,
   focus invisible, saut incohérent ou piège. Tester Entrée et Espace.
3. Vérifier les noms des trois modes, de chaque outil et des croix d’onglets :
   « Fermer [nom du document] » doit identifier le bon document.
4. Ouvrir F1, choisir les trois tailles avec le clavier. Fermer avec le clavier,
   puis rouvrir pour revenir à Normale. Le document original reste inchangé.
5. Vérifier la navigation dans les propriétés de texte, la couleur, la police,
   la taille, l’interligne et les choix d’export. Les valeurs et états doivent
   être annoncés, sans dépendre uniquement d’une couleur.
6. Tester Ctrl+F, les résultats, leur fermeture et le retour du focus. Consigner
   séparément la possibilité réelle de lire et parcourir le contenu du PDF :
   des boutons accessibles ne prouvent pas une lecture accessible du document.
7. Tester F11, Échap, les onglets de documents et la sortie du plein écran.
8. Modifier, annuler, rétablir, enregistrer et rouvrir. Vérifier l’annonce des
   opérations et erreurs, et le retour du focus après chaque dialogue.
9. Vérifier les dialogues de fermeture, y compris Annuler et l’échec de sauvegarde.
10. Sans lancer une récupération automatiquement, ouvrir F1 et demander un
    brouillon disponible. Vérifier son texte, son style et sa sauvegarde séparée.

## Pointage adapté et absence de molette

Utiliser uniquement les commandes visibles, les touches de navigation et les
barres de défilement. Tester un clic maintenu, le glissement lent, l’annulation
par Échap, les miniatures, les poignées et le changement d’écran. Aucune action
ne doit être déclenchée par un simple délai ou par un clic automatique.

## Critères d’acceptation

Pas de commande indispensable hors écran ; fermeture toujours accessible ;
états annoncés ; focus visible et prévisible ; aucun blocage clavier ; aucun
document original modifié par le test. Enregistrer pour chaque étape : réussi,
échec avec reproduction, ou non testé. Ne pas remplacer « non testé » par réussi.

Les imprimantes physiques, Word, les postes verrouillés et l’installation sur
Windows vierge demandent des recettes distinctes. Le setup de test local ne les
certifie pas.
