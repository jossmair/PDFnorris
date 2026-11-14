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
