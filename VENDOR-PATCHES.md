# Correctifs locaux du moteur

Base applicative : PDFnorris V1.05, commit `8ca41a9e3d8eaa94d61b4732fe8203bab802e76a`.
Le paquet fournisseur annonce `pdfcraft-edit 0.9.0`. Le commit amont exact de
l’ensemble de ce snapshot vendored n’est pas établi par cette passe ; ne pas
confondre la base applicative avec un commit PdfCraft amont.

## Suppression groupée de paragraphes

Fichier : `vendor/pdfcraft/crates/edit/src/text.rs`.

- Ajout de `clear_blocks` ; `clear_block` délègue pour un seul bloc.
- Résolution des indices et des scopes avant toute réécriture.
- Conservation des avances TJ et des opérateurs de positionnement.
- Fusion des scopes sélectionnés, limites de sélection et d’imbrication,
  refus d’un scope sélectionné incomplet et des textes utilisés comme découpe.
- Adaptation ciblée dans `src/selection.rs`, sans rasteriser une page.

Preuve : `cargo test --locked --test selection fragmented_overlapping_text`.
La fixture échoue sur la base avant correctif, puis vérifie aperçu, suppression,
annulation/rétablissement et création DOCX/ODT. Les tests de sélection incluent
également une Form textuelle et un scope marqué.

Le test unitaire autonome du workspace fournisseur ne peut pas être lancé en
mode hors ligne avec les entrées disponibles : sa résolution exige une révision
de `craft-libs` non présente dans le cache. Cet échec de préparation est conservé
comme limitation, et non déclaré réussi. Les tests d’intégration PDFnorris
exécutent directement les fonctions modifiées.

SHA-256 du fichier de base : `3071e5a1482cdc5cc5f2441d0b8027760683204e897df6a533edb12b45df49aa`.

SHA-256 du fichier corrigé : `dfc024b79d286ef3f335424f62c6dd97074b8f98ed89c3e3764c53eba7ad36b2`.

Pour une mise à jour fournisseur : comparer le diff à cette base, rechercher
l’équivalent amont, exécuter la régression avant de retirer le correctif, puis
refaire la recette des scopes, du clipping, du rendu et des exports. Conserver
les notices et la provenance des ressources.

## Révisions PDF et mise à niveau progressive

L’adaptateur interne `src/engine.rs` regroupe l’ouverture des octets,
l’inspection de la révision et la sérialisation incrémentale de l’éditeur.
La transaction, ses contrôles de permissions, son historique et la validation
du nombre de pages restent dans `src/document.rs`. Le rendu et les opérations
spécialisées utilisent encore des types PdfCraft.

La révision amont examinée est `4f23ee1c1c2afc6611ca3a71b38162f6259629fb`.
Le premier portage ciblé reprend la suppression de l’ancienne entrée
`/DL` lors de la réécriture d’un flux. Elle décrit la longueur des données
décodées avant édition. La référence du flux partagé et les marques du flux
sont conservées pour les objets non modifiés. Le format de l’API publique
et la version déclarée des crates locales restent ceux du snapshot.
[Implémentation amont de PageStreams](https://github.com/storytold/pdfcraft/blob/4f23ee1c1c2afc6611ca3a71b38162f6259629fb/crates/edit/src/page_streams.rs)

Les protections locales supplémentaires portent sur la suppression d’un texte
servant de masque dans un paragraphe marqué et sur le refus d’éditer un
contenu que le moteur ne peut intégralement décoder. Le décodage strict de
l’inventaire des textes dispose d’un budget cumulé de 16 Mio et de 4 096 flux
par page. Une page légitime dépassant ce budget peut être refusée pour cette
opération. Le lecteur conserve son chemin de rendu habituel.

Pour vérifier le portage, lancer `cargo test --locked --test p0_fidelity`
puis comparer deux exécutables du même exemple `fidelity_probe` avec le
protocole ci-dessous. Refaire ces vérifications pour chaque nouveau portage.

## Comparaison de deux moteurs sur les mêmes documents

Compiler `cargo build --locked --example fidelity_probe` dans chaque révision
avec le même fichier source de sonde. Copier chaque exécutable sous un nom
distinct. Le test utilise Python, pypdf, pypdfium2 et NumPy. ReportLab et
Pillow permettent également de générer les 26 PDF synthétiques.

```powershell
python scripts/p0_fixtures.py fixtures --oracle-dir $oracle
python scripts/p0_corpus.py --sources $sources --limit 469 --phase move --probe $reference --oracle-dir $oracle --output essai-avant
python scripts/p0_corpus.py --sources $sources --cohort essai-avant/report.json --phase move --probe $candidat --oracle-dir $oracle --output essai-apres
python scripts/p0_compare.py essai-avant essai-apres
```

Utiliser des dossiers de sortie neufs en dehors des sources. Le banc dédoublonne
par SHA-256 et vérifie les originaux en fin de traitement. Pour le corpus
synthétique, employer `--sources fixtures --expect-fixtures fixtures/manifest.json`.
La sonde déplace le premier bloc natif admissible des trois premières pages de
3 et 2 points PDF, contrôle les octets après annulation et rétablissement,
puis enregistre et rouvre une copie. PDFium rend toutes les pages, à 1,5 pixel
par point au maximum et au plus 1 400 pixels sur le grand côté. Le contrôle
signale tout pixel hors sélection dont un canal varie de plus de 2 sur 255,
avec 2 points de marge autour du bloc, et compare les positions des glyphes.
Il signale aussi une modification sans effet visuel. pypdf contrôle des
structures de page et de catalogue. Les temps incluent le processus de sonde
et les contrôles indépendants. Pour étudier la vitesse, réserver le poste et
répéter les deux passages en alternant leur ordre.

Conserver les sorties privées en local. Un refus, une différence de rendu,
une erreur du vérificateur et une absence de texte éditable sont des résultats
distincts. Ce parcours échantillonne une édition par document.
