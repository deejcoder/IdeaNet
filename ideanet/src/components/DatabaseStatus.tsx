import React, { useEffect } from 'react';
import { useDatabase } from '../hooks/useDatabase';

const DatabaseStatus: React.FC = () => {
  const {
    isConnected,
    healthMessage,
    isLoading,
    connect,
    disconnect,
    checkHealth,
    error,
  } = useDatabase();

  useEffect(() => {
    // Check health on component mount
    checkHealth();
  }, [checkHealth]);

  const getStatusColor = () => {
    if (isConnected === null) return '#gray';
    return isConnected ? '#4ade80' : '#ef4444';
  };

  const getStatusText = () => {
    if (isConnected === null) return 'Checking...';
    return isConnected ? 'Connected' : 'Disconnected';
  };

  return (
    <div style={{ 
      padding: '20px', 
      border: '1px solid #e5e7eb', 
      borderRadius: '8px', 
      maxWidth: '500px',
      margin: '20px auto'
    }}>
      <h2>Database Status</h2>
      
      <div style={{ marginBottom: '16px' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <div 
            style={{ 
              width: '12px', 
              height: '12px', 
              borderRadius: '50%', 
              backgroundColor: getStatusColor() 
            }}
          />
          <span style={{ fontWeight: 'bold' }}>{getStatusText()}</span>
        </div>
        <p style={{ margin: '8px 0', color: '#6b7280' }}>{healthMessage}</p>
      </div>

      {error && (
        <div style={{ 
          color: '#ef4444', 
          backgroundColor: '#fef2f2', 
          padding: '8px', 
          borderRadius: '4px',
          marginBottom: '16px' 
        }}>
          Error: {error}
        </div>
      )}

      <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
        <button
          onClick={connect}
          disabled={isLoading || isConnected === true}
          style={{
            padding: '8px 16px',
            backgroundColor: isConnected === true ? '#d1d5db' : '#3b82f6',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: isConnected === true ? 'not-allowed' : 'pointer',
          }}
        >
          {isLoading ? 'Connecting...' : 'Connect'}
        </button>

        <button
          onClick={disconnect}
          disabled={isLoading || isConnected === false}
          style={{
            padding: '8px 16px',
            backgroundColor: isConnected === false ? '#d1d5db' : '#ef4444',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: isConnected === false ? 'not-allowed' : 'pointer',
          }}
        >
          {isLoading ? 'Disconnecting...' : 'Disconnect'}
        </button>

        <button
          onClick={checkHealth}
          disabled={isLoading}
          style={{
            padding: '8px 16px',
            backgroundColor: '#6b7280',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
          }}
        >
          Check Health
        </button>
      </div>
    </div>
  );
};

export default DatabaseStatus;
