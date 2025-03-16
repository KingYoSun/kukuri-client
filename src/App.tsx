import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "./components/theme-provider";
import Layout from "./components/layout/Layout";
import HomePage from "./pages/home/HomePage";
import ProfilePage from "./pages/profile/ProfilePage";
import SettingsPage from "./pages/settings/SettingsPage";
import AuthPage from "./pages/auth/AuthPage";
import { Toaster } from "./components/ui/toaster";
import { useAuthStore } from "./stores/auth-store";

// Create a client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      refetchOnWindowFocus: false,
    },
  },
});

function App() {
  const { isAuthenticated } = useAuthStore();

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider defaultTheme="system" storageKey="kukuri-theme">
        <Router>
          {isAuthenticated ? (
            <Layout>
              <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/profile/:userId" element={<ProfilePage />} />
                <Route path="/settings" element={<SettingsPage />} />
              </Routes>
            </Layout>
          ) : (
            <Routes>
              <Route path="*" element={<AuthPage />} />
            </Routes>
          )}
        </Router>
        <Toaster />
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
