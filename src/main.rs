use leptos::*;
use leptos_oidc::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/main.css"/>

        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Router>
            <AppWithRouter/>
        </Router>
       <span> Hello World </span>   
    }
}

fn main() {
    mount_to_body(|| view! { <App />})
}

#[component]
pub fn Home() -> impl IntoView {
    let auth = expect_context::<Auth>();

    view! {
        <Title text="Home"/>
        <h1>Home</h1>

        // Your Pome Page without authentication
    }
}

/// This will be rendered, if the authentication library is still loading
#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <Title text="Loading"/>
        <h1>Loading</h1>

        // Your Loading Page/Animation
    }
}

/// This will be rendered, if the user is unauthenticated
#[component]
pub fn Unauthenticated() -> impl IntoView {
    view! {
        <Title text="Unauthenticated"/>
        <h1>Unauthenticated</h1>

        // Your Unauthenticated Page
    }
}

/// This will be rendered, if the user is authentication
#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <Title text="Profile"/>
        <h1>Profile</h1>

        // Your Profile Page
    }
}

#[component]
pub fn AppWithRouter() -> impl IntoView {
    // Specify OIDC authentication parameters here.
    // Note: This is an example for keycloak, please change it to your needs
    
    // http://localhost:8080/realms/myrealm/protocol/openid-connect/auth?response_type=code&client_id=localdev&redirect_uri=http://localhost:3001/&scope=openid
    let auth_parameters = AuthParameters {
        auth_endpoint:
        "http://localhost:8080/realms/myrealm/protocol/openid-connect/auth"
        .to_string(),
        token_endpoint:
        "http://localhost:8080/realms/myrealm/protocol/openid-connect/token"
        .to_string(),
        logout_endpoint:
        "http://localhost:8080/realms/myrealm/protocol/openid-connect/logout"
        .to_string(),
        client_id: "localdev".to_string(),
        redirect_uri: "http://localhost:3001".to_string(),
        post_logout_redirect_uri: "http://localhost:3001/".to_string(),
        scope: None,
    };

    let auth = Auth::init(auth_parameters);

    view! {
        // This is an example for a navbar where you have a login and logout
        // button, based on the state.
        <div>
            {
               // logging::log!("auth {:?}", auth.authenticated());
            }
            <Authenticated unauthenticated=move || {
                view! {
                    <LoginLink class="text-login">Sign in</LoginLink>
                }
            }>
                <LogoutLink class="text-logut">Sign Out</LogoutLink>
            </Authenticated>
        </div>

        <Routes>
            <Route path="/" view=move || view! { <Home/> }/>

            // This is an example route for your profile, it will render
            // loading if it's still loading, render unauthenticated if it's
            // unauthenticated and it will render the children, if it's
            // authenticated
            <Route
                path="/profile"
                view=move || {
                    view! {
                        <Authenticated
                            loading=move || view! { <Loading/> }
                            unauthenticated=move || view! { <Unauthenticated/> }
                        >
                            <Profile/>
                        </Authenticated>
                    }
                }
            />
        </Routes>
    }
}

