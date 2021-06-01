Name: megadrive-cart-info
Version: 0.1
Release: 1
Summary: Inspect and print Megadrive / Genesis rom headers from images
License: MIT

BuildRequires: rust cargo
Source0: %{name}-%{version}.tar.gz

%description
The longer description for our application

%pre
%prep
# See https://rpm-packaging-guide.github.io/#setup
# so tell setup to expect a different name
%setup

%build
cargo test
cargo build --release

%install
mkdir -p %{buildroot}/%{_bindir}
install -m 0655 target/release/%{name} %{buildroot}/%{_bindir}

%files
%defattr(-, root, root)
%{_bindir}/%{name}
%doc README.md
%license LICENSE

%changelog
* Sun Nov 17 2019 Billy Ray Valentine
- Initial release
