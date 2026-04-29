Se activa cuando un moderador marca un comentario como spam.

### Contexto que recibe el agente

- El comentario, con la marca de post-acción `Is Spam`.
- El **ID del usuario desencadenante** - el moderador que actuó.
- Historial opcional del hilo / del usuario / contexto de la página según esté configurado.

### Quién activa esto

Acción de un moderador humano. Las marcas de spam originadas por agentes (vía [`mark_comment_spam`](#tools-overview)) **no** activan este desencadenador.

### Usos comunes

- **Registro de memoria** - hacer que un agente guarde una nota de memoria sobre el usuario marcado como spam (p. ej., "anteriormente marcado como spam por X por un moderador") para que los futuros agentes de moderación tengan contexto.
- **Aplicación a nivel de usuario** - que un moderador marque un comentario como spam puede ser la señal para que un agente también emita una advertencia o una suspensión breve, condicionada a la aprobación.
- **Espejo en sistemas externos** vía [Webhooks](#webhooks-overview).

---