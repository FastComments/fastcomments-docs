---
Označeni uporabniki bodo prejeli e‑pošto, ki jih obvešča, da so bili označeni ali omenjeni v komentarju.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Vsebina obvestilne e-pošte, ki citira komentar z omembo @winrid v krepki pisavi, ter povezavami za ogled in odjavo'; title='Obvestilo o omenjenem uporabniku' app-screenshot-end]

Izklop obvestil bo preprečil pošiljanje teh e‑pošt, v vsaki e‑pošti pa je vključen glavni del, ki e‑poštnim odjemalcem omogoča, da uporabnika brez težav odjavijo.

Označeni uporabniki prav tako prejmejo obvestilo v aplikaciji, tudi če nimajo shranjenega e‑poštnega naslova.

Če komentar čaka na odobritev moderatorja, se e‑pošta z omembo zadrži in pošlje, ko je komentar odobren. Ne pošlje se, če je komentar označen kot neželen (spam), izbrisan ali ni odobren v 7 dneh.

Če želite preveriti, ali je bil omenjeni uporabnik obveščen po e‑pošti ali zakaj ni bil, odprite [Dnevnike komentarjev](/guide-moderation.html#comment-logs) v razdelku Moderiraj komentarje.