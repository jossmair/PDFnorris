# Ressources graphiques de PDFnorris

## Présentation actuelle du dépôt

- `pdfnorris-logo.png` : visuel fourni par le propriétaire du dépôt pour la présentation GitHub. Copie du fichier fourni, sans retouche. Ce logo de présentation est distinct du visuel incorporé à la V1.01.
- `screenshots/*-v101.png` : cinq captures originales de la V1.01 fournies pour cette présentation ; leur contenu n’a pas été retouché. Voir [leur descriptif](screenshots/README.md).
- `telecharger-installateur.svg` et `telecharger-portable.svg` : boutons vectoriels de téléchargement, avec les liens définis dans le README.

## Ressources incorporées au logiciel

- `pdfnorris.svg`, `pdfnorris.png` et `pdfnorris.ico` : icône orange du logiciel. Le fichier ICO est incorporé à l’exécutable par `build.rs` ; les formats PNG et ICO sont générés par `examples/app_icon.rs`.
- `pdfnorris-welcome.png` : visuel d’accueil fourni pour l’application, conservé avec sa transparence.
- `icons/*.svg` : 58 pictogrammes vectoriels redessinés pour PDFnorris sur une grille de 32 unités. La palette orange et anthracite utilise des reflets et une ombre courte. Les zones de filtre sont explicites pour préserver les traits horizontaux.

Les SVG sont rendus à la résolution effective par `src/icons.rs`. Les tests couvrent leur rendu ainsi que la visibilité des commandes horizontales. Le décodage du logo d’accueil est également testé.

## Éléments historiques

`lecture.png`, `modifier.png`, `pages.png`, `banniere.svg` et `icons-atlas.png` appartiennent à des présentations antérieures. Ils ne sont plus utilisés par le README actuel. L’atlas est un essai généré avec ImageGen, non incorporé au logiciel.

Les attributions et licences des composants tiers restent conservées dans les mentions légales du projet et des distributions.
