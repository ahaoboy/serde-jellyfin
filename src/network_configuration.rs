#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "RequireHttps", skip_serializing_if = "Option:: is_none")]
    pub require_https: Option<bool>,
    #[serde(rename = "CertificatePath", skip_serializing_if = "Option:: is_none")]
    pub certificate_path: Option<String>,
    #[serde(
        rename = "CertificatePassword",
        skip_serializing_if = "Option:: is_none"
    )]
    pub certificate_password: Option<String>,
    #[serde(rename = "BaseUrl", skip_serializing_if = "Option:: is_none")]
    pub base_url: Option<String>,
    #[serde(rename = "PublicHttpsPort", skip_serializing_if = "Option:: is_none")]
    pub public_https_port: Option<f32>,
    #[serde(
        rename = "HttpServerPortNumber",
        skip_serializing_if = "Option:: is_none"
    )]
    pub http_server_port_number: Option<f32>,
    #[serde(rename = "HttpsPortNumber", skip_serializing_if = "Option:: is_none")]
    pub https_port_number: Option<f32>,
    #[serde(rename = "EnableHttps", skip_serializing_if = "Option:: is_none")]
    pub enable_https: Option<bool>,
    #[serde(rename = "PublicPort", skip_serializing_if = "Option:: is_none")]
    pub public_port: Option<f32>,
    #[serde(
        rename = "UPnPCreateHttpPortMap",
        skip_serializing_if = "Option:: is_none"
    )]
    pub u_pn_p_create_http_port_map: Option<bool>,
    #[serde(rename = "UDPPortRange", skip_serializing_if = "Option:: is_none")]
    pub udp_port_range: Option<String>,
    #[serde(rename = "EnableIPV6", skip_serializing_if = "Option:: is_none")]
    pub enable_ipv6: Option<bool>,
    #[serde(rename = "EnableIPV4", skip_serializing_if = "Option:: is_none")]
    pub enable_ipv4: Option<bool>,
    #[serde(rename = "EnableSSDPTracing", skip_serializing_if = "Option:: is_none")]
    pub enable_ssdp_tracing: Option<bool>,
    #[serde(rename = "SSDPTracingFilter", skip_serializing_if = "Option:: is_none")]
    pub ssdp_tracing_filter: Option<String>,
    #[serde(rename = "UDPSendCount", skip_serializing_if = "Option:: is_none")]
    pub udp_send_count: Option<f32>,
    #[serde(rename = "UDPSendDelay", skip_serializing_if = "Option:: is_none")]
    pub udp_send_delay: Option<f32>,
    #[serde(
        rename = "IgnoreVirtualInterfaces",
        skip_serializing_if = "Option:: is_none"
    )]
    pub ignore_virtual_interfaces: Option<bool>,
    #[serde(
        rename = "VirtualInterfaceNames",
        skip_serializing_if = "Option:: is_none"
    )]
    pub virtual_interface_names: Option<String>,
    #[serde(
        rename = "GatewayMonitorPeriod",
        skip_serializing_if = "Option:: is_none"
    )]
    pub gateway_monitor_period: Option<f32>,
    #[serde(
        rename = "EnableMultiSocketBinding",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_multi_socket_binding: Option<bool>,
    #[serde(
        rename = "TrustAllIP6Interfaces",
        skip_serializing_if = "Option:: is_none"
    )]
    pub trust_all_ip6_interfaces: Option<bool>,
    #[serde(
        rename = "HDHomerunPortRange",
        skip_serializing_if = "Option:: is_none"
    )]
    pub hd_homerun_port_range: Option<String>,
    #[serde(
        rename = "PublishedServerUriBySubnet",
        skip_serializing_if = "Option:: is_none"
    )]
    pub published_server_uri_by_subnet: Option<Vec<String>>,
    #[serde(
        rename = "AutoDiscoveryTracing",
        skip_serializing_if = "Option:: is_none"
    )]
    pub auto_discovery_tracing: Option<bool>,
    #[serde(rename = "AutoDiscovery", skip_serializing_if = "Option:: is_none")]
    pub auto_discovery: Option<bool>,
    #[serde(rename = "RemoteIPFilter", skip_serializing_if = "Option:: is_none")]
    pub remote_ip_filter: Option<Vec<String>>,
    #[serde(
        rename = "IsRemoteIPFilterBlacklist",
        skip_serializing_if = "Option:: is_none"
    )]
    pub is_remote_ip_filter_blacklist: Option<bool>,
    #[serde(rename = "EnableUPnP", skip_serializing_if = "Option:: is_none")]
    pub enable_u_pn_p: Option<bool>,
    #[serde(
        rename = "EnableRemoteAccess",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_remote_access: Option<bool>,
    #[serde(
        rename = "LocalNetworkSubnets",
        skip_serializing_if = "Option:: is_none"
    )]
    pub local_network_subnets: Option<Vec<String>>,
    #[serde(
        rename = "LocalNetworkAddresses",
        skip_serializing_if = "Option:: is_none"
    )]
    pub local_network_addresses: Option<Vec<String>>,
    #[serde(rename = "KnownProxies", skip_serializing_if = "Option:: is_none")]
    pub known_proxies: Option<Vec<String>>,
    #[serde(
        rename = "EnablePublishedServerUriByRequest",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_published_server_uri_by_request: Option<bool>,
}
