Name:       rusty_d_bus
Version:    0.1.0
Release:    0
Summary:    RPM package
Group:      Applications/System
License:    MIT

%description
Example D-Bus service written in Rust.

%prep

%build

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin/
mkdir -p %{buildroot}/etc/dbus-1/system.d/
mkdir -p %{buildroot}/%{_datadir}/dbus-1/system-services/

install -m 744 $SOURCES_ROOT/target/release/rusty_d_bus %{buildroot}/usr/bin/rusty_d_bus
install $SOURCES_ROOT/d-bus/org.example.rusty.conf %{buildroot}/etc/dbus-1/system.d/
install $SOURCES_ROOT/d-bus/org.example.rusty.service %{buildroot}/%{_datadir}/dbus-1/system-services/

%files
/usr/bin/rusty_d_bus
%{_sysconfdir}/dbus-1/system.d/org.example.rusty.conf
%{_datadir}/dbus-1/system-services/org.example.rusty.service

%changelog

%post
systemctl daemon-reload || true

%preun
