import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface DatabaseHealth {
  connected: boolean;
  message: string;
}

interface UseDatabaseReturn {
  isConnected: boolean | null;
  healthMessage: string;
  isLoading: boolean;
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
  checkHealth: () => Promise<void>;
  error: string | null;
}

// Check if we're running in Tauri context
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

export const useDatabase = (): UseDatabaseReturn => {
  const [isConnected, setIsConnected] = useState<boolean | null>(null);
  const [healthMessage, setHealthMessage] = useState<string>(
    isTauri ? 'Unknown' : 'Please use the desktop app (not browser)'
  );
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(
    isTauri ? null : 'Tauri APIs not available in browser - use the desktop app'
  );

  const checkHealth = useCallback(async () => {
    if (!isTauri) {
      setError('Tauri APIs not available - please use the desktop app');
      setIsConnected(false);
      return;
    }
    
    try {
      setError(null);
      const health = await invoke<DatabaseHealth>('db_health');
      setIsConnected(health.connected);
      setHealthMessage(health.message);
    } catch (err) {
      setError(typeof err === 'string' ? err : String(err));
      setIsConnected(false);
      setHealthMessage('Health check failed');
    }
  }, []);

  const connect = useCallback(async () => {
    if (!isTauri) {
      setError('Tauri APIs not available - please use the desktop app');
      return;
    }
    
    try {
      setIsLoading(true);
      setError(null);
      await invoke('db_connect');
      await checkHealth();
    } catch (err) {
      setError(typeof err === 'string' ? err : String(err));
      setIsConnected(false);
    } finally {
      setIsLoading(false);
    }
  }, [checkHealth]);

  const disconnect = useCallback(async () => {
    if (!isTauri) {
      setError('Tauri APIs not available - please use the desktop app');
      return;
    }
    
    try {
      setIsLoading(true);
      setError(null);
      await invoke('db_disconnect');
      await checkHealth();
    } catch (err) {
      setError(typeof err === 'string' ? err : String(err));
    } finally {
      setIsLoading(false);
    }
  }, [checkHealth]);

  return {
    isConnected,
    healthMessage,
    isLoading,
    connect,
    disconnect,
    checkHealth,
    error,
  };
};
