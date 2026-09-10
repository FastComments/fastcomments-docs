## Connectez votre compte

1. Dans Zapier, ajoutez une étape FastComments à un Zap, ou ouvrez la page de l'application FastComments dans le répertoire d'applications Zapier.  
2. Choisissez **Sign in to FastComments**. Zapier vous demande d'abord votre région : choisissez **United States** sauf si votre compte a été créé sur la région UE (`eu.fastcomments.com`).  
3. Une fenêtre FastComments s'ouvre. Connectez‑vous si vous n'êtes pas déjà connecté.  
4. Examinez la page de consentement. Elle montre l'application Zapier, le compte auquel elle sera connectée, et les autorisations demandées (lecture et écriture). Choisissez **Approve**.  
5. Zapier enregistre la connexion et l'étiquette avec le nom de votre site et votre nom d'utilisateur.  

La connexion utilise OAuth. Aucune clé API n'est copiée dans Zapier, et le jeton détenu par Zapier ne fonctionne que pour le compte que vous avez approuvé.

## Qui peut se connecter

La personne qui approuve la connexion doit être un **API admin** sur le compte FastComments. Les propriétaires de compte ont cette permission ; d'autres membres de l'équipe peuvent se la voir attribuer sur la page Utilisateurs. Quelqu'un qui ne l'a pas voit une page « you do not have permission » au lieu du formulaire de consentement.

## Connecter le bon site

La page de consentement connecte le compte auquel vous êtes actuellement connecté. Si vous gérez plusieurs comptes, passez au bon depuis le sélecteur de compte avant d'approuver, ou utilisez le lien **switch account** sur la page de consentement. L'étiquette de connexion dans Zapier affiche le nom du site, donc un mauvais choix est facile à repérer.

## Examiner et révoquer l'accès

Chaque connexion apparaît sous **Connected Apps** dans le tableau de bord FastComments, avec les autorisations qu'elle possède et la dernière fois qu'elle a été utilisée. La révoquer là déconnecte Zapier immédiatement ; tout Zap utilisant cette connexion s'arrête jusqu'à ce qu'elle soit reconnectée. Vous pouvez également supprimer la connexion du côté Zapier sous **My Apps**.