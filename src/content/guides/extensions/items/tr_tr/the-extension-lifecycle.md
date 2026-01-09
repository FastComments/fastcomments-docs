Her uzantı için betik, yorum bileşeni ilk yorum kümesini almaya ve kullanıcı arayüzünü oluşturmaya başlamadan önce alınır ve çalıştırılır.

İlk yüklemede, aşağıdaki veriler uzantı nesnesine eklenecektir:

- `config` - `config` nesnesine bir referans.
- `translations` - `translations` nesnesine bir referans.
- `commentsById` - id'ye göre tüm yorumların bir referansı.
- `root` - kök DOM düğümüne bir referans.

Uzantılar, yorum bileşeninin uygun zamanlarda çağıracağı istenen işlevleri geçersiz kılmalıdır.