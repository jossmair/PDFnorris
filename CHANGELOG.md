# PDFnorris — notes de version

## V1.04

- Panneau de réglages : les commandes de repli et de réouverture utilisent un fond crème discret, un contour gris fin et un chevron orange, adaptés au panneau clair.

## V1.03

- Panneau de réglages : deux pictogrammes vectoriels dédiés remplacent les boutons textuels de repli et de réouverture, avec des détails cuivrés et une surface anthracite.
- Les commandes conservent une info-bulle et un nom accessible. Le repli préserve l’outil actif et l’adaptation de la page à l’espace disponible.
- Poursuite des contrôles locaux OCR et DOCX/ODT ; les documents du corpus restent privés.

## V1.02

- OCR : rétablissement des coordonnées après le redressement Windows, conservation du texte clair sur fond sombre et nettoyage limité aux pixels des lettres.
- OCR : les fragments isolés incertains dans les titres décoratifs conservent leurs pixels d’origine.
- Miniatures : poignée de défilement plus fine, sans recouvrir les pages ; la gouttière disparaît lorsque toutes les miniatures tiennent dans le panneau.
- Icônes : nouvelle famille de 58 pictogrammes vectoriels, recréée sur une grille commune, avec contours fins, surfaces anthracite et détails cuivrés.
- Historique : budget de 512 Mio pour les tampons PDF des révisions, partagé entre les onglets, en complément des 100 étapes maximum par document. Les étapes les plus anciennes sont supprimées en premier ; le document courant reste intact. Ce budget ne limite pas la mémoire totale du logiciel.
- Exports Office : les numéros encadrés, les libellés de questions et les bordures de cellules ne sont plus interprétés comme des fractions, des exposants ou des soulignements.
- Exports Office : conservation des séparateurs de page et isolation des fractions entre colonnes.
- Les aperçus de tracés denses et leur suppression conservent les limites de décodage de page, sans appliquer le budget de réplication réservé aux déplacements.

## V1.00

Première version sous le nom PDFnorris, issue du code de PDFinator 1.0.10.

- Nouvelle identité orange et anthracite, icône Windows et véritable exécutable PDFnorris.
- Slogan : « Aucun PDF ne lui résiste. »
- Lecture, modification, OCR local, annotations, dessin et gestion des pages.
- Exports DOCX/ODT avec paragraphes et tableaux modifiables ; les blocs mathématiques complexes peuvent être conservés comme images.
- Portable et installeur Windows x64.

Les marqueurs internes des anciens documents et les brouillons de récupération restent compatibles.

## Améliorations après V1.00

- Nouvelle famille de 58 icônes orange et anthracite, avec relief discret.
- Maintenir le bouton central de la souris déplace temporairement la vue, sans changer l’outil actif.
- Une page isolée est centrée verticalement lorsqu’elle tient dans la vue.
- Dans Modifier/Dessin, cliquer dans une cellule d’un tableau dessiné permet d’y saisir du texte ; les bordures restent sélectionnables.

- Correctif du déplacement au bouton central pour les pages entièrement visibles, sans quitter l’outil courant.
- Correctif du rendu des traits horizontaux (réduction de fenêtre et zoom moins).
- Confirmation de fermeture intégrée au thème de l’application, avec sauvegarde, abandon explicite ou annulation.

## V1.01

- Le bouton central est capturé à l’entrée native des événements, avant les contrôles de l’interface. Le maintien du bouton déplace la vue sans modifier le document ni changer l’outil.
- Les fichiers de distribution et les informations Windows portent la version technique 1.0.1.
