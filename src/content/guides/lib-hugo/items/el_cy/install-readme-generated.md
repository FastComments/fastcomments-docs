Επιλέξτε έναν από τους δύο τυπικούς τρόπους για να προσθέσετε ένα στοιχείο θέματος Hugo.

### Επιλογή Α: Μονάδα Hugo (συνιστάται)

Από τη ρίζα του ιστότοπού σας:

```bash
hugo mod init github.com/you/your-site   # only if your site is not already a module
hugo mod get github.com/FastComments/fastcomments-hugo
```

Στη συνέχεια, προσθέστε την εισαγωγή στο `hugo.toml` σας:

```toml
[module]
  [[module.imports]]
    path = "github.com/FastComments/fastcomments-hugo"
```

### Επιλογή Β: Στοιχείο θέματος (Git submodule)

```bash
git submodule add https://github.com/FastComments/fastcomments-hugo.git themes/fastcomments-hugo
```

Στη συνέχεια, αναφέρετέ το από το `hugo.toml` σας. Καταχωρήστε το δίπλα στο δικό σας θέμα· οι μεταγενέστερες εγγραφές υπερισχύουν, οπότε κρατήστε πρώτα το θέμα σας:

```toml
theme = ["your-theme", "fastcomments-hugo"]
```