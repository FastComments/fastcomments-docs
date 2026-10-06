Les utilisateurs tagués recevront un e‑mail les informant qu’ils ont été tagués ou mentionnés dans un commentaire.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Corps de l\'e-mail de notification citant un commentaire avec la mention @winrid en gras, plus les liens de visualisation et de désabonnement'; title='Notification d\'utilisateur mentionné' app-screenshot-end]

Désactiver les notifications empêchera ces e‑mails, et un en‑tête est fourni dans chaque e‑mail afin que les clients de messagerie puissent permettre à l’utilisateur de se désabonner facilement.

Les utilisateurs tagués reçoivent également une notification dans l’application, même s’ils n’ont aucune adresse e‑mail enregistrée.

Si le commentaire attend l’approbation d’un modérateur, l’e‑mail de mention est retenu et envoyé une fois le commentaire approuvé. Il n’est pas envoyé si le commentaire est marqué comme spam, supprimé, ou non approuvé dans les 7 jours.

Pour voir si un utilisateur mentionné a reçu un e‑mail, ou pourquoi il ne l’a pas reçu, ouvrez les [Journaux de commentaires](/guide-moderation.html#comment-logs) du commentaire depuis la page Modérer les commentaires.