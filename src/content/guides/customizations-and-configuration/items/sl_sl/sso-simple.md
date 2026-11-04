[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Z enostavnim SSO lahko zagotovimo pripomočku za komentiranje informacije o uporabniku, tako da ne mora vpisati svojega uporabniškega imena ali e‑naslova za komentiranje.

Enostavni SSO lahko konfiguriramo na naslednji način:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Enostavni SSO'; code-example-end]

Uporabnik bo prijavljen in v ozadju bo ustvaril SSO uporabnika. Uporabnik bo imel `createdFromSimpleSSO` nastavljen na `true`, če je pridobljen iz API-ja.

Opombe: 

- E‑naslov je edinstveni identifikator za enostavni SSO.
- Zagotovitev e‑naslova pri enostavnem SSO ni obvezna, vendar bodo njihovi komentarji privzeto prikazani kot "Neoverjeni". <b>Če e‑naslov ni naveden, uporabnik ne more biti popolnoma avtenticiran.</b>
- **NOVO** Od januarja 2022: Uporabniška imena ne morajo biti edinstvena po celotnem fastcomments.com
- Enostavni SSO lahko samodejno ustvari in posodobi SSO uporabnike, če je naveden e‑naslov, in uporabnik ni bil prvotno ustvarjen iz varnega SSO. Če enostavni SSO paket poimenuje uporabnika, ustvarjenega z varnim SSO ali API-jem, ostane ta uporabnik nedotaknjen, obiskovalec pa dobi ločenega uporabnika.
- Z lastnostjo `badgeConfig` lahko določite značke za uporabnika. Polje `badgeIds` vsebuje ID-je globalnih značkov, ki se povežejo z uporabnikom. Polje `pageBadgeIds` vsebuje ID-je značkov, omejenih na trenutno stran (`urlId`) — ti znački so prikazani le na strani, kjer so dodeljeni. Če je `override` nastavljeno na `true`, bo nadomestilo obstoječe prikazane značke (globalni in stranični znački se neodvisno prepišejo); če je `false`, se bodo dodali k obstoječim značkom.

#### Nastavitve enostavnega SSO

Enostavni SSO je vedno na voljo. Dve nastavitvi nadzorujeta, koliko FastComments mu zaupa. Najdete ju lahko z iskanjem SSO v <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">pravilu prilagoditve</a>. Obe sta privzeto izklopljeni in se uporabljata ne glede na to, ali je "Zahtevaj SSO za ogled komentarjev" vklopljeno.

Računi, ki so že uporabljali enostavni SSO, ko so bile te nastavitve dodane, imata obe vklopljeni, zato se za njih ni nič spremenilo.

Velja le pravilo prilagoditve, ki se najbolj ujema s stranjo, zato če imate več pravil, nastavite te na vsakem od njih. Novo pravilo se začne z vklopljenimi nastavitvami, če je katerokoli od vaših drugih pravil imel nastavitve vklopljene.

##### Samodejno potrdi komentarje prek enostavnega SSO

Ko je to vklopljeno, so komentarji uporabnikov enostavnega SSO takoj potrjeni.

Ko je to izklopljeno, se komentarji uporabnikov enostavnega SSO prikažejo kot "Neoverjeni", dokler uporabnik ne klikne povezave, ki jo pošljemo na e‑naslov v paketu enostavnega SSO. Če samodejno odobrite le potrjene komentarje, tudi ti čakajo na to povezavo. Če v paketu ni e‑naslova, komentar ostane neoverjen.

Ta nastavitev vpliva le na komentarje. Glasovi uporabnikov enostavnega SSO se vedno štejejo takoj.

##### Dovoli admine enostavnega SSO

Ko je to vklopljeno, lahko uporabnik enostavnega SSO postane skrbnik ali moderator vašega računa:

- Uporabnik enostavnega SSO, ki ima pravice skrbnika ali moderatorja, jih ohrani.
- Uporabnik enostavnega SSO z enakim e‑naslovom kot eden od vaših skrbnikov ali moderatorjev prevzame njihove pravice.

Ko je to izklopljeno, enostavni SSO ne dodeli teh pravic. Paket enostavnega SSO, ki poimenuje uporabnika s pravicami, ustvari ločenega uporabnika brez pravic, izvirni uporabnik pa ostane nedotaknjen.

Paketi enostavnega SSO niso podpisani, zato ko je to vklopljeno, lahko kdorkoli, ki pozna e‑naslov ali ID skrbnika, deluje v njegovem imenu. Če je to pomembno za vaše spletno mesto, pustite to izklopljeno in uporabite varni SSO za svoje skrbnike.

Uporabnikov, ustvarjenih z varnim SSO ali API-jem, enostavni SSO ne more doseči, ne glede na to, ali je to vklopljeno.

---