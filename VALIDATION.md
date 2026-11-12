# Validation de PDFnorris

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

