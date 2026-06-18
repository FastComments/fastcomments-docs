---
Bir Hugo tema bileşeni eklemek için iki standart yöntemden birini seçin.

### Seçenek A: Hugo Modülü (önerilen)

Sitenizin kök dizininden:

```bash
hugo mod init github.com/you/your-site   # yalnızca siteniz henüz bir modül değilse
hugo mod get github.com/FastComments/fastcomments-hugo
```

Ardından import'u `hugo.toml` dosyanıza ekleyin:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Seçenek B: Tema bileşeni (Git alt modülü)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Ardından bunu `hugo.toml` dosyanızdan referanslayın. Kendi temanızla birlikte listeleyin; sonraki girdiler kazanır, bu yüzden temanızı ilk sırada tutun:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```
---