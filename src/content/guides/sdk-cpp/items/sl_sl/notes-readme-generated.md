### Broadcast Ids

Videli boste, da je pri nekaterih API klicih treba posredovati `broadcastId`. Ko prejmete dogodke, boste ta ID prejeli nazaj, tako da lahko dogodek prezrete, če nameravate spremembe optimistično uporabiti na odjemalcu
(kar verjetno želite, saj ponuja najboljšo izkušnjo). Tukaj posredujte UUID. ID mora biti dovolj edinstven, da se v eni seji brskalnika ne pojavi dvakrat.

### SSO (Single Sign-On)

Za primere SSO glejte spodaj.