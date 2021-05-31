Name: megadrive-cart-info
Version: 0.1
Release: 1
Summary: Inspect and print Megadrive / Genesis rom headers from images
License: MIT

#Url: https://github.com/billyrayvalentine/python-megadrive-cart-info
BuildRequires: rust cargo
Source0: %{name}-%{version}.tar.gz
#Requires: python3

%description
The longer description for out application

%pre
#getent group %{name} >/dev/null || groupadd -r %{name}
#getent passwd %{name} >/dev/null || useradd -r -g %{name} -d /usr/local/%{name} -s /sbin/nologin -c "System user for %{name}" %{name}

%prep
# See https://rpm-packaging-guide.github.io/#setup
# so tell setup to expect a different name
%setup

%build
cargo build -release

%install
mkdir -p %{buildroot}/usr/local/%{name}
mkdir -p %{buildroot}/usr/local/bin
#mkdir -p %{buildroot}/{%_sysconfdir}
#mkdir -p %{buildroot}/%{_unitdir}
#mkdir -p %{buildroot}/%{_sysconfdir}/sysconfig
#mkdir -p %{buildroot}/var/log/%{name}

chmod 0755 %{buildroot}/usr/local/%{name}
install -m 0644 src/*.py %{buildroot}/usr/local/%{name}
#install -m 0644 megadrive-cart-info.conf %{buildroot}/%{_sysconfdir}
#install -m 0644 %{name}.service %{buildroot}/%{_unitdir}
#install -m 0644 %{name}.sysconfig %{buildroot}/%{_sysconfdir}/sysconfig/%{name}

%files
%defattr(-, root, root)
/usr/local/%{name}/*.py
#%{_sysconfdir}/megadrive-cart-info.conf
#%{_sysconfdir}/sysconfig
#%_unitdir
%doc README.md
#%attr(0775,%{name},%{name}) /var/log/%{name}/

%changelog
* Sun Nov 17 2019 Billy Ray Valentine
- Initial release
