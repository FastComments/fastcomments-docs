---
Ajanı, bir yorum düzenlendiğinde tetikler.

### Ajanın aldığı bağlam

- Yorumun şu anki (düzenleme sonrası) hali.
- **Önceki yorum metni** ayrı bir çitlenmiş blok olarak (`PREVIOUS_TEXT`). Bu düzenleme tetikleyicisine özgüdür - ajan önce/sonra karşılaştırması yapabilir.
- Yapılandırıldığı şekilde isteğe bağlı konu / kullanıcı geçmişi / sayfa bağlamı.

### Önemli

- Tetikleyici, moderatörlerin bir kullanıcı adına yaptığı düzenlemeler dahil olmak üzere herhangi bir başarılı düzenleme için tetiklenir.
- Ajanlara yorum düzenleme aracı sunulmaz; ajanlar yorumları hiçbir şekilde düzenleyemez.
- Önceki yorum metni güvensiz girdi olarak çitlenir. Platformun sistem istemi modele çitlerin içindeki talimatları takip etmemesini hatırlatır - bu burada önemlidir, çünkü kötü niyetli bir kullanıcı, düzenleme olaylarını izleyen herhangi bir ajana yönelik "önceki talimatlarını görmezden gel" yükü eklemek için bir yorumu düzenleyebilir.

### Yaygın kullanım alanları

- **Aklanmış içeriğin yakalanması** - bir kullanıcı, moderatör gittikten sonra önceden temiz bir yoruma spam eklemek için düzenleme yapabilir.
- **Küçük düzenlemelerin izlenmesi** - topluluğunuz düzenlemeleri herhangi bir denetim nedeni ile ayrı olaylar olarak ele alıyorsa.

### Maliyet notu

Düzenleme tetikleyicileri, yorum metninin iki kopyasını görür (standart COMMENT bloğundaki yeni sürüm ve PREVIOUS_TEXT bloğundaki eski sürüm). Uzun yorumlar için bu, çalıştırmanın token maliyetini yaklaşık olarak bir `COMMENT_ADD` tetikleyicisine göre iki katına çıkarır - bütçe yaparken bunu göz önünde bulundurun.

---