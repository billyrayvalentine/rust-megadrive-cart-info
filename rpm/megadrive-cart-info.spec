Name: megadrive-cart-info
Version: 0.1
Release: 1
Summary: Inspect and print Megadrive / Genesis ROM headers from images
License: MIT
Url: https://github.com/billyrayvalentine/rust-megadrive-cart-info
Group: Productivity/Other
Source0: %{name}-%{version}.tar.gz
BuildRequires: rust cargo

%description
Simple tool to inspect Megadrive / Genesis ROM images from the commandline.

%global debug_package %{nil}

%prep
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
* Tue Jun 01 2021 Billy Ray Valentine
- Initial release
