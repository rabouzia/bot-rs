first thing we launch the website, i open a burp browser to see and understand all the http header

i put the following query =>
``` sql
query {
  __schema {
    types {
      name
      description
    }
  }
}
```

we go a result, this is the first breach, the introspection is activated


GraphQL introspection activée — La requête __schema est accessible publiquement, ce qui permet d’énumérer tout le schéma et facilite la recherche de vecteurs d’attaque (exposition du modèle de données et des mutations). Impact : fuite d’informations sensibles. Sévérité : haute.

Mass assignment possible — La mutation newUser(userInput:{ role: "admin" }) pourrait permettre d’ajouter des champs non filtrés (par ex. role) lors de la création d’un compte, entraînant une montée de privilèges. Impact : élévation de privilèges. Sévérité : moyenne/élevée.

Upload pouvant mener à RCE — L’endpoint uploadUserAvatar accepte des fichiers ; sans vérification stricte du contenu et du chemin, il est possible d’uploader un fichier exécutable (ex. .php) et de l’exécuter via le serveur. Impact : exécution de code à distance. Sévérité : critique.

Champs sensibles exposés (password, secret, iban, credit_card) — Le type User contient des champs tels que password, resetPasswordToken, secret, iban et credit_card ; si ces champs sont récupérables via des queries, cela provoque une fuite de données hautement sensibles. Impact : compromission d’identifiants et données financières. Sévérité : haute.

Différences d’erreur en authentification (user enumeration) — La mutation authentification renvoie des messages/temps différents selon que l’utilisateur existe ou non, permettant d’énumérer des comptes valides. Impact : cartographie d’utilisateurs pour phishing/brute-force. Sévérité : moyenne.

Modification de solde sans contrôle (updateBalance) — La mutation updateBalance(user_id: "2", amount: 9999) peut, si non protégée, permettre de modifier le solde d’un autre utilisateur et de voler des fonds. Impact : fraude financière directe. Sévérité : critique.

IDOR sur transactions (from_id contrôlable) — newTransaction(transaction: { from_id: "1", to_id: "2", amount: ... }) : si from_id est contrôlable sans vérification d’autorisation, on peut initier des transferts depuis des comptes tiers. Impact : vol par détournement de transaction. Sévérité : haute.

Token de reset réutilisable / prévisible (resetPassword) — Si resetPasswordToken est réutilisable, prévisible ou non lié correctement à l’utilisateur, il est possible de prendre le contrôle d’un compte via la mutation resetPassword. Impact : takeover de comptes. Sévérité : haute.

Fuite OSINT via champs utilisateurs — Les champs exposés (User.address, email, job_title, last_ip) permettent de profiler les utilisateurs et de préparer des attaques d’ingénierie sociale ciblées (phishing, spear-phishing). Impact : risques de compromission humaine. Sévérité : moyenne.

Fuite de métadonnées par les uploads — Les fichiers uploadés par uploadUserAvatar peuvent contenir des métadonnées (EXIF, noms d’ordinateur, IP) et ainsi divulguer des informations privées sur les utilisateurs. Impact : fuite d’informations privées/techniques. Sévérité : faible à moyenne.

Introspection et erreurs verbeuses facilitent la découverte — La combinaison introspection + messages d’erreur détaillés (stack traces, SQL messages) facilite l’identification rapide de champs, types et points faibles exploitables. Impact : accélération du travail de reconnaissance et d’exploitation. Sévérité : haute.

Injection possible dans les scalars JSON / inputs — L’argument info: JSON de addUserInfo ou des champs memo/address peuvent être vecteurs d’injection (NoSQL/SQL/Template/Command) si le parsing n’est pas sécurisé. Impact : exfiltration, exécution ou corruption de données. Sévérité : moyenne à élevée.

Fuite de stacktrace et informations d’implémentation — Les réponses d’erreur exposent des stacktraces Python complètes (chemins absolus, modules et lignes de code, traces d’exécution), par exemple :

Traceback ... /root/.cache/... graphql/execution/execute.py ... TypeError: Cannot return null for non-nullable field Mutation.updateBalance.


Cela permet d’identifier la stack (Python + librairie GraphQL), les chemins d’installation (virtualenv/poetry) et des points de plantage précis. Impact : facilitation du fingerprinting, recherche d’exploits ciblés et accélération de la reconnaissance. Sévérité : élevée. Remédiation rapide : masquer les stacktraces côté client (formatter d’erreurs), désactiver le mode debug en production et journaliser la stacktrace côté serveur uniquement.