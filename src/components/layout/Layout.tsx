import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { useAuthStore } from '@/stores/auth-store';
import { Button } from '@/components/ui/button';
import { useTheme } from '@/components/theme-provider';
import { MoonIcon, SunIcon, HomeIcon, PersonIcon, GearIcon } from '@radix-ui/react-icons';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const location = useLocation();
  const { user, logout } = useAuthStore();
  const { theme, setTheme } = useTheme();

  return (
    <div className="min-h-screen bg-background">
      <header className="sticky top-0 z-10 border-b bg-background/95 backdrop-blur">
        <div className="container flex h-14 items-center">
          <div className="mr-4 flex">
            <Link to="/" className="flex items-center space-x-2">
              <span className="font-bold text-xl">Kukuri</span>
            </Link>
            <span className="text-xs text-muted-foreground ml-2">
              Choose your connections, control your network
            </span>
          </div>
          <div className="flex-1"></div>
          <nav className="flex items-center space-x-4">
            <Button variant="ghost" size="icon" onClick={() => setTheme(theme === "dark" ? "light" : "dark")}>
              {theme === "dark" ? <SunIcon className="h-4 w-4" /> : <MoonIcon className="h-4 w-4" />}
              <span className="sr-only">Toggle theme</span>
            </Button>
          </nav>
        </div>
      </header>
      <div className="container flex-1 items-start md:grid md:grid-cols-[220px_minmax(0,1fr)] md:gap-6 lg:grid-cols-[240px_minmax(0,1fr)] lg:gap-10">
        <aside className="fixed top-14 z-30 -ml-2 hidden h-[calc(100vh-3.5rem)] w-full shrink-0 md:sticky md:block">
          <div className="h-full py-6 pr-6 lg:py-8">
            <nav className="flex flex-col space-y-2">
              <Link to="/">
                <Button
                  variant={location.pathname === "/" ? "secondary" : "ghost"}
                  className="w-full justify-start"
                >
                  <HomeIcon className="mr-2 h-4 w-4" />
                  Home
                </Button>
              </Link>
              {user && (
                <Link to={`/profile/${user.id}`}>
                  <Button
                    variant={location.pathname.startsWith("/profile") ? "secondary" : "ghost"}
                    className="w-full justify-start"
                  >
                    <PersonIcon className="mr-2 h-4 w-4" />
                    Profile
                  </Button>
                </Link>
              )}
              <Link to="/settings">
                <Button
                  variant={location.pathname === "/settings" ? "secondary" : "ghost"}
                  className="w-full justify-start"
                >
                  <GearIcon className="mr-2 h-4 w-4" />
                  Settings
                </Button>
              </Link>
              <Button
                variant="ghost"
                className="w-full justify-start text-red-500 hover:text-red-600 hover:bg-red-100 dark:hover:bg-red-900/20"
                onClick={logout}
              >
                Logout
              </Button>
            </nav>
          </div>
        </aside>
        <main className="flex w-full flex-col overflow-hidden py-6">
          {children}
        </main>
      </div>
    </div>
  );
};

export default Layout;