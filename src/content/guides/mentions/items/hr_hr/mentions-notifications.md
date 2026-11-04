Označeni korisnici primit će e-mail kojim se obavještavaju da su označeni, odnosno spomenuti, u komentaru.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Tijelo e-maila s obavijesti koje citira komentar s podebljanim @winrid spomenom, uz poveznice za pregled i odjavu'; title='Obavijest o spomenutom korisniku' app-screenshot-end]

Isključivanje obavijesti spriječit će ove e-poruke, a svaki e-mail sadrži zaglavlje kako bi klijenti e-pošte mogli korisniku omogućiti nesmetanu odjavu s obavijesti.

Označeni korisnici također dobivaju obavijest unutar aplikacije, čak i kada nemaju unesenu adresu e-pošte.

Ako komentar čeka odobrenje moderatora, e-mail sa spomenom se zadržava i šalje se nakon što je komentar odobren. Ne šalje se ako je komentar označen kao spam, izbrisan ili nije odobren u roku od 7 dana.

Da biste vidjeli je li spomenuti korisnik dobio e-mail ili zašto nije, otvorite [Zapisnike komentara](/guide-moderation.html#comment-logs) komentara sa stranice Moderiraj komentare.