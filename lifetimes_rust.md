# Guide des Lifetimes en Rust

## Le problème de base

Quand une struct stocke une **référence** vers une autre donnée, Rust doit s'assurer que cette référence reste toujours valide. Les lifetimes sont le mécanisme qui garantit qu'on ne peut jamais avoir un pointeur vers des données détruites.

## Exemple dans notre code : AuthStatement

### La struct AuthStatement stocke une référence

```rust
pub struct AuthStatement<'a> {
  auth_context: &'a mut AuthContext,  // Référence, pas une copie !
  is_empty: bool,
  ok_so_far: bool,
  error: Option<AuthenticationError>,
}
```

Le `<'a>` est un **paramètre de lifetime**. Il dit : "Cette struct contient une référence qui doit rester valide pendant toute la durée `'a`".

### Scénario problématique sans lifetimes

```rust
let statement = {
    let mut context = AuthContext::new(...).await;
    context.authorize()  // On crée un AuthStatement
}; // ← context est détruit ici !

// Mais statement existe encore et pointe vers... quoi ?! 💥
```

Grâce aux lifetimes, **Rust refuse de compiler** ce code dangereux !

## impl<'a> AuthStatement<'a>

Quand une struct a un paramètre générique (type ou lifetime), il faut le **redéclarer** dans le bloc `impl` :

```rust
impl<'a> AuthStatement<'a> {
     ↑                  ↑
     |                  |
  déclare 'a       utilise 'a
}
```

C'est comme pour les types génériques :
```rust
struct MyVec<T> { ... }
impl<T> MyVec<T> { ... }
     ↑        ↑
```

## La fonction authorize

```rust
pub fn authorize(&'_ mut self) -> AuthStatement<'_> {
    AuthStatement::new(self)
}
```

### Décomposition

**`&'_ mut self`** :
- Référence mutable vers `self` (l'AuthContext)
- `'_` = "Rust, déduis toi-même le lifetime"
- Appelons ce lifetime "durée de l'emprunt"

**`-> AuthStatement<'_>`** :
- Retourne un AuthStatement
- `<'_>` = "Rust, déduis le lifetime"
- Rust comprend : "L'AuthStatement vit aussi longtemps que l'emprunt de self"

### Les deux `'_` sont liés !

```rust
&'_ mut self  →  AuthStatement<'_>
     ↑                        ↑
     |________________________|
          même lifetime !
```

Version explicite équivalente :
```rust
pub fn authorize<'a>(&'a mut self) -> AuthStatement<'a>
```

## Ce que garantissent les lifetimes

1. Tant que `AuthStatement` existe, l'`AuthContext` ne peut pas être détruit
2. Tant que `AuthStatement` existe, l'`AuthContext` ne peut pas être utilisé autrement (il est "emprunté")
3. Quand `AuthStatement` est détruit, l'emprunt se termine et on peut réutiliser `AuthContext`

## En résumé

- **`struct AuthStatement<'a>`** : déclare qu'elle contient des références avec lifetime `'a`
- **`impl<'a> AuthStatement<'a>`** : déclare qu'on implémente des méthodes pour cette struct générique
- **`&'_ mut self` et `AuthStatement<'_>`** : les lifetimes sont déduits automatiquement par Rust, et il comprend qu'ils doivent être identiques

Les lifetimes = **sécurité mémoire garantie à la compilation** 🦀
