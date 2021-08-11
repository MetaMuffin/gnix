pkgname=gnix
pkgver=1
pkgrel=1
pkgdesc="a bad reverse proxy"
arch=('any')
url="https://github.com/metamuffin/gnix.git"
license=('AGPL-3.0-only')

source=("$pkgname"::'git+https://github.com/metamuffin/gnix.git')
sha256sums=(SKIP)

build() {
    cd "$srcdir/$pkgname"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname"
    install -Dm 755 "target/release/gnix" "$pkgdir/usr/bin/gnix"
    install -Dm 644 "gnix.service" "$pkgdir/usr/lib/systemd"
}
