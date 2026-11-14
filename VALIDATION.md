# Validation de PDFnorris

## En préparation — lecture plein écran

Les tests d’interface simulée couvrent l’apparition et la rétraction des commandes,
la stabilité du cadrage au survol, la croix de sortie, F11 et Échap, le changement
d’onglet et la manipulation réelle du curseur vertical sans modifier les octets PDF.
Le contrôle Clippy strict passe. La bascule native de fenêtre reste à confirmer
par une utilisation interactive sur le poste.

## V1.04 — icônes sur fond clair

Le rendu des commandes est vérifié sur fond clair. Le test de repli et de
réouverture couvre la conservation des bords de page aux trois largeurs.
Les paquets et les contrôles GitHub doivent être validés avant publication.

## V1.03 — panneau de réglages

Les contrôles simulés vérifient le repli et la réouverture par leurs noms accessibles,
la conservation de l’outil et du zoom, ainsi que la visibilité des bords en mode
largeur et page entière sur trois tailles de fenêtre. La famille compte 60 SVG.
Les tests de corpus OCR et exports restent locaux ; leur réussite technique
ne constitue pas une garantie de fidélité pour chaque document.

## V1.02 — OCR, historique et conversion structurée

264 tests automatisés passent localement. Le diagnostic privé optionnel reste
ignoré dans le lot standard. Le formatage et Clippy strict sont contrôlés.
Les régressions synthétiques couvrent notamment les coordonnées OCR redressées,
le texte clair sur photographie, les numéros encadrés, les fractions entre
colonnes, les bordures de cellules et les séparateurs de page.

Le budget des tampons PDF d’historique est testé entre onglets, avec conservation
des étapes contiguës d’annulation et de rétablissement. Il ne représente pas
un plafond de mémoire totale du processus.

Le ZIP portable contient l’exécutable natif x64 et le fichier légal consolidé.
Son intégrité, ses icônes et ses empreintes sont vérifiées. Le setup est installé
puis désinstallé en mode de test dans un dossier temporaire avec espaces et accent,
sans créer de raccourcis ni d’entrée de désinstallation sur le poste.

Les rendus Office sont examinés avec LibreOffice sous Windows. Une validation
complète avec Microsoft Word, imprimantes réelles et lecteurs d’écran reste à
réaliser. Les documents et résultats détaillés du corpus ne sont pas publiés.
Les contrôles GitHub Windows et RustSec doivent réussir avant publication.

## V1.01 — contrôle du bouton central et interface

114 tests de l’application passent localement, avec Clippy strict. Ils couvrent
notamment le traitement natif du bouton central, la navigation sur une page
entièrement visible, le décodage du logo PNG, le centrage d’une page isolée,
la saisie dans les cellules et les confirmations de fermeture.
Le ZIP et le cycle installation/désinstallation du setup sont vérifiés.
Le test physique du bouton central avec la souris de l’utilisateur reste à confirmer.
Les contrôles GitHub Windows et RustSec doivent réussir avant publication.


## V1.00 — édition, OCR et exports

**243 tests automatisés réussis.** Les régressions synthétiques couvrent les
encodages Type 1 incorporés et leur priorité vis-à-vis d’Encoding/ToUnicode,
les caractères superposés, le déplacement des couches de faux gras, les
chiffres colorés, les exposants, les pieds de page et les polices ODT.
Les tests sur les encodages et les superpositions ont échoué avant leur correctif.

Les vérifications locales portent sur la lecture, les modifications, les
déplacements, l’enregistrement, la réouverture, l’annulation et le rétablissement.
Des contrôles indépendants et des comparaisons visuelles complètent les tests.
Les documents, les rendus et les résultats détaillés du corpus restent locaux.

Les illustrations gardent leur position sur la page et les tableaux restent
structurés. Les formules complexes sont conservées à partir du rendu original :
ces blocs sont des images dans le fichier Office. Le texte courant et les
tableaux restent modifiables ; les fractions simples compatibles peuvent
rester des équations natives. Une régression compare exactement les pixels
d’un bloc mathématique avec son rendu source et vérifie que la prose et les
tableaux restent natifs.

L’OCR conserve les zones décoratives manifestement incertaines dans l’image.
Ces zones ne deviennent donc pas automatiquement du texte éditable.
Les conversions sont contrôlées avec LibreOffice ; cela ne remplace pas une
recette avec Microsoft Word et ne garantit pas une reconstruction identique
de tout PDF.

Le portable est vérifié après extraction, y compris l’icône Windows. Le setup
est installé et désinstallé dans un dossier isolé, avec comparaison des fichiers.
Ces tests ne remplacent pas une recette sur chaque configuration Windows,
avec un lecteur d’écran ou avec de véritables imprimantes.

