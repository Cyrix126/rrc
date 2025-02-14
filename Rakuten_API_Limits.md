# Rakuten API Limits

This document is informing all the limitations of the Rakuten API.

## Non immediate updates

All requests sent to the API can take time to apply, contrary to the direct requests to the website.
The API possess endpoints to check the progress of actions.

## Frequency of calls

The call to overwrite products must not be used more than one time per hour for <40K products. If products are more than 40K, add a delay of an hour for every 10K products (example: for 60K, limit requests to 1 per 3 hours). [^1]

10 file per hour is the maximum. It is advised to set a delay of 10 minutes minimum between each files sent.[^2]

## Size limit

The size limit of a file is 30Mo[^3]

## Diseappearing orders

The new orders endpoint does not store orders more than 2 days older.[^4]

[^1]: https://outils.fr.shopping.rakuten.com/dev-pro/fr/documentation/Gestion_de_stock/Importer_votre_inventaire_a_l_aide_de_flux_XML/Importer_son_flux_XML_GenericImportFile_Rakuten_France_Webservices.html 

[^2]: https://outils.fr.shopping.rakuten.com/dev-pro/faqs/quel-est-la-frequence-maximale-denvoi-de-fichier/

[^3]: https://outils.fr.shopping.rakuten.com/dev-pro/faqs/quel-est-la-limite-de-poids-dun-fichier/

[^4]: https://outils.fr.shopping.rakuten.com/dev-pro/fr/documentation/Nouvelles_ventes/Importer_les_nouvelles_commandes_GetNewSales_Rakuten_France_Webservices.html
