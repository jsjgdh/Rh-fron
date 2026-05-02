//! Authentication and authorization module for role-based access control.
//!
//! Defines roles, permissions, and utility functions for checking access rights.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// User roles in the system.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "System Administrator")]
    SystemAdministrator,
    #[serde(rename = "Policy Architect")]
    PolicyArchitect,
    #[serde(rename = "Operator")]
    Operator,
    #[serde(rename = "Auditor")]
    Auditor,
}

impl Role {
    /// Parse a role from string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "System Administrator" => Some(Role::SystemAdministrator),
            "Policy Architect" => Some(Role::PolicyArchitect),
            "Operator" => Some(Role::Operator),
            "Auditor" => Some(Role::Auditor),
            _ => None,
        }
    }

    /// Get string representation of the role.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::SystemAdministrator => "System Administrator",
            Role::PolicyArchitect => "Policy Architect",
            Role::Operator => "Operator",
            Role::Auditor => "Auditor",
        }
    }
}

/// Permissions that can be granted to users.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    // Dashboard permissions
    #[serde(rename = "dashboard.view")]
    DashboardView,

    // Workflow permissions
    #[serde(rename = "workflows.view")]
    WorkflowsView,
    #[serde(rename = "workflows.create")]
    WorkflowsCreate,
    #[serde(rename = "workflows.edit")]
    WorkflowsEdit,
    #[serde(rename = "workflows.delete")]
    WorkflowsDelete,

    // Execution permissions
    #[serde(rename = "executions.view")]
    ExecutionsView,
    #[serde(rename = "executions.run")]
    ExecutionsRun,
    #[serde(rename = "executions.simulate")]
    ExecutionsSimulate,

    // Integration permissions
    #[serde(rename = "integrations.view")]
    IntegrationsView,
    #[serde(rename = "integrations.configure")]
    IntegrationsConfigure,
    #[serde(rename = "integrations.test")]
    IntegrationsTest,

    // Webhook permissions
    #[serde(rename = "webhooks.view")]
    WebhooksView,
    #[serde(rename = "webhooks.manage")]
    WebhooksManage,

    // Audit log permissions
    #[serde(rename = "audit_logs.view")]
    AuditLogsView,
    #[serde(rename = "audit_logs.export")]
    AuditLogsExport,

    // User management permissions
    #[serde(rename = "users.view")]
    UsersView,
    #[serde(rename = "users.manage")]
    UsersManage,
    #[serde(rename = "users.invite")]
    UsersInvite,

    // Settings permissions
    #[serde(rename = "settings.view")]
    SettingsView,
    #[serde(rename = "settings.edit")]
    SettingsEdit,

    // Organization permissions
    #[serde(rename = "organization.view")]
    OrganizationView,
    #[serde(rename = "organization.manage")]
    OrganizationManage,
}

impl Permission {
    /// Get all permissions for a given role.
    pub fn for_role(role: &Role) -> HashSet<Permission> {
        let mut perms = HashSet::new();

        match role {
            Role::SystemAdministrator => {
                // Full access - all permissions
                perms.insert(Permission::DashboardView);
                perms.insert(Permission::WorkflowsView);
                perms.insert(Permission::WorkflowsCreate);
                perms.insert(Permission::WorkflowsEdit);
                perms.insert(Permission::WorkflowsDelete);
                perms.insert(Permission::ExecutionsView);
                perms.insert(Permission::ExecutionsRun);
                perms.insert(Permission::ExecutionsSimulate);
                perms.insert(Permission::IntegrationsView);
                perms.insert(Permission::IntegrationsConfigure);
                perms.insert(Permission::IntegrationsTest);
                perms.insert(Permission::WebhooksView);
                perms.insert(Permission::WebhooksManage);
                perms.insert(Permission::AuditLogsView);
                perms.insert(Permission::AuditLogsExport);
                perms.insert(Permission::UsersView);
                perms.insert(Permission::UsersManage);
                perms.insert(Permission::UsersInvite);
                perms.insert(Permission::SettingsView);
                perms.insert(Permission::SettingsEdit);
                perms.insert(Permission::OrganizationView);
                perms.insert(Permission::OrganizationManage);
            }
            Role::PolicyArchitect => {
                // Workflows, integrations - full access
                perms.insert(Permission::DashboardView);
                perms.insert(Permission::WorkflowsView);
                perms.insert(Permission::WorkflowsCreate);
                perms.insert(Permission::WorkflowsEdit);
                perms.insert(Permission::WorkflowsDelete);
                perms.insert(Permission::ExecutionsView);
                perms.insert(Permission::ExecutionsRun);
                perms.insert(Permission::ExecutionsSimulate);
                perms.insert(Permission::IntegrationsView);
                perms.insert(Permission::IntegrationsConfigure);
                perms.insert(Permission::IntegrationsTest);
                perms.insert(Permission::WebhooksView);
                perms.insert(Permission::WebhooksManage);
                perms.insert(Permission::AuditLogsView);
                perms.insert(Permission::SettingsView);
                perms.insert(Permission::SettingsEdit);
                perms.insert(Permission::OrganizationView);
            }
            Role::Operator => {
                // Executions, monitoring - can run and view
                perms.insert(Permission::DashboardView);
                perms.insert(Permission::WorkflowsView);
                perms.insert(Permission::ExecutionsView);
                perms.insert(Permission::ExecutionsRun);
                perms.insert(Permission::IntegrationsView);
                perms.insert(Permission::WebhooksView);
                perms.insert(Permission::AuditLogsView);
                perms.insert(Permission::SettingsView);
                perms.insert(Permission::OrganizationView);
            }
            Role::Auditor => {
                // Read-only access to audit logs and view-only on most things
                perms.insert(Permission::DashboardView);
                perms.insert(Permission::WorkflowsView);
                perms.insert(Permission::ExecutionsView);
                perms.insert(Permission::IntegrationsView);
                perms.insert(Permission::WebhooksView);
                perms.insert(Permission::AuditLogsView);
                perms.insert(Permission::AuditLogsExport);
                perms.insert(Permission::UsersView);
                perms.insert(Permission::OrganizationView);
            }
        }

        perms
    }
}

/// Navigation item with associated permission requirements.
#[derive(Debug, Clone)]
pub struct NavItem {
    pub label: &'static str,
    pub route: &'static str,
    pub required_permission: Permission,
    pub icon: Option<&'static str>,
    pub children: Vec<NavItem>,
}

impl NavItem {
    /// Create a simple nav item.
    pub fn new(label: &'static str, route: &'static str, permission: Permission) -> Self {
        Self {
            label,
            route,
            required_permission: permission,
            icon: None,
            children: Vec::new(),
        }
    }

    /// Create a nav item with an icon.
    pub fn with_icon(
        label: &'static str,
        route: &'static str,
        permission: Permission,
        icon: &'static str,
    ) -> Self {
        Self {
            label,
            route,
            required_permission: permission,
            icon: Some(icon),
            children: Vec::new(),
        }
    }

    /// Add child items.
    pub fn with_children(mut self, children: Vec<NavItem>) -> Self {
        self.children = children;
        self
    }
}

/// Get all navigation items organized by section.
pub fn get_navigation_structure() -> Vec<(&'static str, Vec<NavItem>)> {
    vec![
        (
            "Console",
            vec![NavItem::new("Console", "/dashboard", Permission::DashboardView)],
        ),
        (
            "Studio",
            vec![
                NavItem::new("Studio", "/upload", Permission::WorkflowsCreate),
                NavItem::new("Workflow Editor", "/workflow/:name/:version/edit", Permission::WorkflowsEdit),
            ],
        ),
        (
            "Forensics",
            vec![
                NavItem::new("Visualizer", "/visualize", Permission::ExecutionsView),
                NavItem::new("History", "/history", Permission::ExecutionsView),
                NavItem::new("Ledger", "/ledger", Permission::WorkflowsView),
            ],
        ),
        (
            "Management",
            vec![
                NavItem::new("Integrations", "/integrations", Permission::IntegrationsView),
                NavItem::new("Settings", "/settings", Permission::SettingsView),
            ],
        ),
        (
            "Support",
            vec![NavItem::new("Docs", "/docs", Permission::DashboardView)],
        ),
    ]
}

/// Check if user has a specific permission.
pub fn has_permission(user_role: Option<&str>, permission: &Permission) -> bool {
    let role = match user_role {
        Some(r) => match Role::from_str(r) {
            Some(role) => role,
            None => return false,
        },
        None => return false,
    };

    let permissions = Permission::for_role(&role);
    permissions.contains(permission)
}

/// Check if user has any of the given permissions.
pub fn has_any_permission(user_role: Option<&str>, permissions: &[Permission]) -> bool {
    permissions.iter().any(|p| has_permission(user_role, p))
}

/// Check if user has all of the given permissions.
pub fn has_all_permissions(user_role: Option<&str>, permissions: &[Permission]) -> bool {
    permissions.iter().all(|p| has_permission(user_role, p))
}

/// Get all permissions for the user's role.
pub fn get_user_permissions(user_role: Option<&str>) -> HashSet<Permission> {
    match user_role {
        Some(r) => match Role::from_str(r) {
            Some(role) => Permission::for_role(&role),
            None => HashSet::new(),
        },
        None => HashSet::new(),
    }
}

/// Check if the user can view a specific route.
pub fn can_access_route(user_role: Option<&str>, route: &str) -> bool {
    // Map routes to required permissions
    let required = match route {
        "/dashboard" => Some(Permission::DashboardView),
        "/upload" => Some(Permission::WorkflowsCreate),
        "/visualize" => Some(Permission::ExecutionsView),
        "/history" => Some(Permission::ExecutionsView),
        "/ledger" => Some(Permission::WorkflowsView),
        "/integrations" => Some(Permission::IntegrationsView),
        "/settings" => Some(Permission::SettingsView),
        "/docs" => Some(Permission::DashboardView),
        "/user-manual" => Some(Permission::DashboardView),
        r if r.starts_with("/execution/") => Some(Permission::ExecutionsView),
        r if r.starts_with("/workflow/") && r.contains("/edit") => Some(Permission::WorkflowsEdit),
        r if r.starts_with("/workflow/") => Some(Permission::WorkflowsView),
        _ => None,
    };

    match required {
        Some(perm) => has_permission(user_role, &perm),
        None => true, // Routes without explicit permission requirements are accessible
    }
}

/// Permission checker component props for Dioxus.
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionCheckProps {
    pub user_role: Option<String>,
    pub required_permission: Permission,
    pub children: dioxus::prelude::Element,
}

/// Get display name for a permission (for UI).
pub fn permission_display_name(permission: &Permission) -> &'static str {
    match permission {
        Permission::DashboardView => "View Dashboard",
        Permission::WorkflowsView => "View Workflows",
        Permission::WorkflowsCreate => "Create Workflows",
        Permission::WorkflowsEdit => "Edit Workflows",
        Permission::WorkflowsDelete => "Delete Workflows",
        Permission::ExecutionsView => "View Executions",
        Permission::ExecutionsRun => "Run Executions",
        Permission::ExecutionsSimulate => "Simulate Executions",
        Permission::IntegrationsView => "View Integrations",
        Permission::IntegrationsConfigure => "Configure Integrations",
        Permission::IntegrationsTest => "Test Integrations",
        Permission::WebhooksView => "View Webhooks",
        Permission::WebhooksManage => "Manage Webhooks",
        Permission::AuditLogsView => "View Audit Logs",
        Permission::AuditLogsExport => "Export Audit Logs",
        Permission::UsersView => "View Users",
        Permission::UsersManage => "Manage Users",
        Permission::UsersInvite => "Invite Users",
        Permission::SettingsView => "View Settings",
        Permission::SettingsEdit => "Edit Settings",
        Permission::OrganizationView => "View Organization",
        Permission::OrganizationManage => "Manage Organization",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_from_str() {
        assert_eq!(
            Role::from_str("System Administrator"),
            Some(Role::SystemAdministrator)
        );
        assert_eq!(
            Role::from_str("Policy Architect"),
            Some(Role::PolicyArchitect)
        );
        assert_eq!(Role::from_str("Operator"), Some(Role::Operator));
        assert_eq!(Role::from_str("Auditor"), Some(Role::Auditor));
        assert_eq!(Role::from_str("Invalid"), None);
    }

    #[test]
    fn test_admin_has_all_permissions() {
        let perms = Permission::for_role(&Role::SystemAdministrator);
        assert!(perms.contains(&Permission::DashboardView));
        assert!(perms.contains(&Permission::WorkflowsCreate));
        assert!(perms.contains(&Permission::UsersManage));
        assert!(perms.contains(&Permission::AuditLogsView));
    }

    #[test]
    fn test_auditor_read_only() {
        let perms = Permission::for_role(&Role::Auditor);
        assert!(perms.contains(&Permission::AuditLogsView));
        assert!(perms.contains(&Permission::WorkflowsView));
        assert!(!perms.contains(&Permission::WorkflowsCreate));
        assert!(!perms.contains(&Permission::ExecutionsRun));
        assert!(!perms.contains(&Permission::UsersManage));
    }

    #[test]
    fn test_operator_can_run_executions() {
        let perms = Permission::for_role(&Role::Operator);
        assert!(perms.contains(&Permission::ExecutionsView));
        assert!(perms.contains(&Permission::ExecutionsRun));
        assert!(!perms.contains(&Permission::WorkflowsCreate));
        assert!(!perms.contains(&Permission::UsersManage));
    }

    #[test]
    fn test_architect_workflow_permissions() {
        let perms = Permission::for_role(&Role::PolicyArchitect);
        assert!(perms.contains(&Permission::WorkflowsCreate));
        assert!(perms.contains(&Permission::WorkflowsEdit));
        assert!(perms.contains(&Permission::WorkflowsDelete));
        assert!(perms.contains(&Permission::IntegrationsConfigure));
        assert!(!perms.contains(&Permission::UsersManage));
    }
}
