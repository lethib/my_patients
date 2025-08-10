import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "@/api/api";
import { Login } from "./Login";

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Login />
    </QueryClientProvider>
  );
}

export default App;
