---
Korisnici koji su označeni dobiće e‑mail koji ih obaveštava da su označeni ili pomenuti u komentaru.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Telo e‑mail obaveštenja koje citira komentar sa @winrid pomenom podebljanim, plus linkove za pregled i odjavu'; title='User Mentioned Notification' app-screenshot-end]

Isključivanje obaveštenja će sprečiti slanje ovih e‑mailova, a zaglavlje je uključeno u svaki e‑mail kako bi klijenti e‑maila omogućili korisniku da se jednostavno odjavi.

Korisnici koji su označeni takođe dobijaju obaveštenje u aplikaciji, čak i kada nemaju registrovanu e‑mail adresu.

Ako je komentar na čekanju za odobrenje moderatora, e‑mail sa pomenom se zadržava i šalje kada se komentar odobri. Ne šalje se ako je komentar označen kao spam, obrisan ili nije odobren u roku od 7 dana.

Da biste videli da li je pomenuti korisnik dobio e‑mail ili zašto nije, otvorite [Dnevnik komentara](/guide-moderation.html#comment-logs) komentara sa stranice Moderiraj komentare.