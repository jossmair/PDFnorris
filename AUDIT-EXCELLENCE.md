# Audit de stabilisation — 14 novembre 2026

## Périmètre et statut

La V1.05 a diffusé le plein écran. Le lot de stabilisation est versionné V1.06.
Les résultats locaux ci-dessous sont distincts de la validation GitHub exigée
avant sa publication et des recettes matérielles qui restent à effectuer.

Cette passe associe un inventaire des sources applicatives, des tests, scripts,
workflows et sources vendored, une lecture ciblée des chemins de sauvegarde,
récupération, sélection, conversion, rendu et commandes, et les rapports locaux
du corpus. **L’inventaire ne prouve pas une revue sémantique exhaustive du code
vendored.** Aucun fichier personnel, chemin de corpus ou résultat détaillé du
corpus n’est publié ici. Les originaux sont préservés.

## Défauts confirmés et corrections locales

| Constat | Correction et preuve |
| --- | --- |
| La taille des commandes n’avait pas de réglage global dédié. | F1 propose Normale, Grande et Très grande. Le réglage utilise l’échelle globale egui, séparée de la commande de zoom PDF, et se mémorise atomiquement dans les données locales de l’utilisateur. Un test choisit Grande avec la souris et vérifie la préférence, le zoom manuel et les octets PDF. Le mode d’ajustement automatique peut recalculer son zoom quand la surface disponible change. |
| Le panneau F1 pouvait déborder après ajout de réglages. | Les explications défilent, les réglages et la fermeture restent séparés. Tests de présence et de visibilité de la fermeture sur des surfaces logiques de 760×540, 640×360 et 426×240. Cela ne constitue pas une recette DPI Windows. |
| Les croix d’onglets annonçaient seulement « Fermer ». | Noms accessibles « Fermer [document] », sans afficher ce texte dans l’onglet. Vérification AccessKit sur un document synthétique. |
| Les brouillons étaient conservés en production mais la reprise volontaire n’avait pas de commande dans l’aide. | F1 ouvre la récupération seulement à la demande et seulement si des copies existent. Aucun dialogue ni restauration automatique au lancement. Test du parcours et de la conservation du document ouvert. Le chemin historique PDFinator est conservé. |
| Le cache des textures était borné en nombre, pas en octets. | Éviction hors écran au-delà de 128 Mio pour les pages et 16 Mio pour les miniatures, en complément des plafonds en nombre. La vue visible reste conservée. Test déclenchant l’éviction sous le plafond en nombre. |
| Un fichier de préférences anormalement gros pouvait être lu sans limite propre. | Lecture bornée à 4 Kio, abandon des valeurs inconnues ou invalides, retour au réglage normal. Test des cas absent, invalide, trop gros et valide. |

Le budget de textures est une limite de rétention du cache, **pas un plafond de
RAM ou de mémoire GPU du processus**. Les textures visibles peuvent dépasser le
budget ; les buffers de rendu, les documents, les onglets et les polices ont
leurs propres coûts. Les textures partagées rendent les sommes conservatrices.

Les préférences d’interface sont par utilisateur, y compris pour le portable.
Elles ne contiennent aucun document. Leur portabilité entre postes et une
commande explicite d’effacement restent à traiter.

## Correctifs complémentaires vérifiés

- Le refus « the paragraph no longer exists » est reproduit sur une fixture
  synthétique de texte fragmenté et superposé. La régression échoue avant le
  correctif. `clear_blocks` résout les paragraphes dans un inventaire unique,
  conserve les avances textuelles et refuse toujours les découpes non gérables.
  Les anciens appels unitaires restent compatibles. Annulation/rétablissement,
  aperçu et création des deux formats Office sont testés.
- Les conversions qui présentaient cette erreur sont relancées localement : elles
  produisent désormais leurs archives. Leur XML et leur intégrité sont contrôlés.
  Un cas corrigé s’ouvre dans LibreOffice, en DOCX et ODT, sur une seule page.
  Cela ne garantit pas la mise en page de tous les documents.
- La lecture des PDF ajoutés, leur analyse et la fusion passent dans un worker.
  Le document affiché reste disponible. La validation finale n’effectue ni lecture
  disque, ni analyse PDF, ni sérialisation sur le fil d’interface.
- Un résultat préparé est lié à l’instance et aux octets d’origine. Une modification
  concurrente le rend obsolète et provoque un refus explicite. Un changement d’onglet
  conserve le résultat pour sa destination ; une annulation ne modifie aucun PDF.
  L’application de l’ajout constitue une seule étape d’historique et se rouvre.
- Les sources d’une fusion sont limitées cumulativement à 256 Mio. Les dépassements
  sont refusés, sans relever la limite d’ouverture. Les transferts par glissement
  entre onglets et certaines réorganisations restent synchrones : ne pas déclarer
  toutes les opérations de pages asynchrones.
- Le correctif fournisseur est décrit dans `VENDOR-PATCHES.md`. Aucun changement
  massif de moteur ni nouvelle dépendance n’est introduit.

## Éléments déjà vérifiés, sans nouveau correctif artificiel

- Les écritures utilisent un temporaire voisin, synchronisé avant remplacement.
- Le moteur refuse les modifications interdites par les permissions et demande
  une acceptation explicite pour les documents signés.
- Les brouillons de production sont activés après construction du contrôleur.
  Le contrôleur désactivé utilisé dans les tests ne signifiait pas une absence
  de journalisation en production.
- L’export Office s’exécute dans un worker avec contrôle d’annulation et de
  validation finale. L’OCR propose déjà une revue par sa commande de propriétés,
  en complément du double-clic direct demandé pour le parcours courant.
- L’historique possède déjà un budget de tampons PDF partagé entre onglets.
- La limite d’ouverture de 256 Mio reste inchangée.

## Fidélité : obstacles effectivement reproduits

Les rapports locaux comportent encore des refus liés aux paragraphes, au texte
utilisé comme découpe, à la géométrie invalide et aux pages très riches en tracés.
Les rendus LibreOffice montrent aussi des écarts de pagination et de disposition.
Une archive DOCX/ODT valide ne prouve donc pas une conversion fidèle.

Aucun contournement par image de page entière n’a été ajouté. Aucun échec n’a
été transformé en succès pour améliorer une métrique. Les conversions complexes
restent un chantier prioritaire : reproduire chaque erreur dans une fixture
synthétique, corriger, puis contrôler le rendu indépendant et la réouverture.
Le défaut d’inventaire de suppression est corrigé dans ce lot ; les autres
classes de refus ne sont pas assimilées à ce même défaut.

## Mesures de référence

Machine de développement courante, processus de diagnostic optimisé figé,
trois exécutions par taille. Le script reproductible est `scripts/benchmark_open.py` avec les arguments
`--probe` (exécutable corpus_probe optimisé) et `--output` (dossier de résultats).
Le diagnostic ouvre puis analyse le texte de la
première page. Les fichiers sont des PDF synthétiques légers ; le document de
500 pages pèse environ 169 ko. Le probe provient de la version 1.0.3, avec le
même moteur PDF que la V1.05 ; il ne mesure pas l’interface.

| Pages | Médiane ouverture + analyse première page, processus inclus |
| ---: | ---: |
| 1 | 12,48 ms |
| 10 | 11,36 ms |
| 100 | 13,84 ms |
| 300 | 20,07 ms |
| 500 | 30,55 ms |

Ces relevés ont été pris sur une machine de développement également utilisée
pour les compilations. Ils ne forment pas un banc isolé ; refaire les relevés
à charge identique avant toute conclusion d’optimisation.

Le premier lancement à une page a pris environ 502 ms ; les suivants environ
12 ms. Cette dispersion est conservée dans le rapport local. Ces nombres ne
mesurent ni la première page visible, ni la fluidité, ni le pic mémoire, ni les
gros scans. Aucun gain chronométrique du moteur n’est revendiqué pour ce lot.

Comparaison dans un même binaire optimisé, sur un document synthétique de
150 paragraphes, avec suppression de 149 paragraphes :

| Essai | Appels séquentiels | Traitement groupé |
| --- | ---: | ---: |
| 1 | 191,19 ms | 1,71 ms |
| 2 | 158,78 ms | 1,49 ms |
| 3 | 133,06 ms | 1,24 ms |

Le texte et les rectangles restants concordent à chaque essai. La médiane passe
de 158,78 à 1,49 ms dans ce scénario. Le benchmark reproduit l’ancien parcours
séquentiel avec les appels unitaires et le compare à l’appel groupé dans le même
moteur ; ce n’est pas une comparaison de toute l’application entre deux versions.
Commande reproductible : `cargo run --release --example selection_benchmark`.
Aucun gain global de fluidité, de première page visible ou de RSS n’en est déduit.

Avant/après déterministe du test de cache : trois textures de page de 256 octets
restaient sous le plafond de douze objets ; avec un budget de test de 512 octets,
les pages hors écran sont évincées et la texture visible de 256 octets reste.
Ce test ne représente pas une mesure de RSS ou de mémoire GPU réelle.

## Tests et recette

- V1.05 : 267 tests locaux, fmt et Clippy strict ; CI Windows et RustSec réussie.
  Portable et installation/désinstallation du setup vérifiés.
- Lot V1.06 : 278 tests passent, un diagnostic privé optionnel reste ignoré ;
  fmt et Clippy strict passent. L’inventaire légal public conserve ses chemins
  relatifs. Aucune dépendance nouvelle n’est ajoutée.
- La CI sur le commit final doit réussir avant publication. Une limite d’utilisation
  a temporairement bloqué les opérations externes pendant cette passe.
- Aucun nouveau rendu interactif Windows ou comparatif de captures n’est annoncé.
  Les tests d’interface simulée ne remplacent pas un contrôle visuel physique.
- La recette NVDA/DPI reproductible est dans [ACCESSIBILITE-RECETTE.md](ACCESSIBILITE-RECETTE.md).
  NVDA, Word, imprimantes réelles, poste vierge, ordinateur ancien et postes
  verrouillés restent non validés dans cette passe.

## Parcours et simplicité

Lecture de code et simulations disponibles, sans étude chronométrée avec débutants :
ouverture par Ctrl+O ; correction par Modifier puis clic dans le texte ; ajout de
texte par cadre glissé ; propriétés contextuelles ; surlignage et commentaires
en Lecture ; insertion d’image par outil dédié ; cellules des tableaux dessinés ;
pages par glissement et menu contextuel ; fusion par ajout de PDF ; copie par
Ctrl+C/Ctrl+V ; enregistrement par Ctrl+S ; choix DOCX/ODT dans l’export ; impression
par Ctrl+P. Les boîtes de fichier et pilotes influencent les gestes réels.

Le lot n’ajoute pas de menu permanent. F1 contient les choix de taille et l’accès
volontaire aux brouillons. Un nombre de clics fiable pour tous ces parcours,
leur ordre de tabulation complet et la découverte des objets par un débutant
demandent encore une recette interactive ; aucun chiffre de clics n’est inventé.

## Réévaluation des six domaines

Les notes initiales sont celles de la mission, pas des mesures reproduites ici.
Une nouvelle note numérique précise serait insuffisamment étayée par cette passe.

| Domaine | Référence de la mission | Réévaluation fondée sur les preuves disponibles |
| --- | ---: | --- |
| Architecture | 8,7 | Objectif 9,6 non établi : workers distincts, transactions présentes ; gros modules et certains transferts synchrones restent à traiter. |
| Design | 8,4 | Objectif 9,6 non établi : plein écran corrigé et identité préservée ; recette visuelle multiscale manquante. |
| Ergonomie | 8,1 | Objectif 9,6 non établi : parcours contextuels présents ; mesure avec utilisateurs et navigation clavier complète manquantes. |
| Tests/maintenance | 9,0 | Objectif 9,7 non établi : nouvelles régressions locales et CI V1.05 verte ; CI du lot et stress/fuzzing élargi à réaliser. |
| Fidélité/compatibilité | 7,4 | Objectif 9,6 non atteint : écarts Office et refus complexes reproduits, Word non validé. |
| Accessibilité | 6,8 | Objectif 9,6 non établi : tailles, noms et panneau améliorés ; NVDA, DPI réel, cibles et accès au contenu documentaire à valider. |

## Suite prioritaire

1. Reproduire et corriger les échecs Office/édition complexes avec comparaison indépendante.
2. Passer les gros transferts de pages en arrière-plan avec état transactionnel,
   annulation et protection contre un changement du document pendant le travail.
3. Exécuter la matrice NVDA/DPI et corriger les blocages du clavier et du contenu.
4. Instrumenter première page visible, latences UI, RSS et pic mémoire sur des
   scans lourds, puis comparer les optimisations sur la même machine.
5. Compléter les tests de fichiers hostiles, la provenance précise des correctifs
   vendored et les recettes Word, imprimantes et postes d’établissement.

**Conclusion : stabilisation concrète, objectifs 9,5+ non certifiés.** La mission
d’excellence n’est pas considérée terminée par ce premier lot.
