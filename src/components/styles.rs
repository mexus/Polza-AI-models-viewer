use dioxus::prelude::*;

#[component]
pub fn GlobalStyles() -> Element {
    rsx! {
        style {
            "
            .filter-input-container {{
                position: relative;
                display: flex;
                align-items: center;
            }}

            .filter-input {{
                width: 100%;
                padding: 12px;
                padding-right: 40px;
                font-size: 16px;
                border: 2px solid #e0e0e0;
                border-radius: 6px;
                box-sizing: border-box;
                transition: border-color 0.2s;
            }}

            .filter-input:focus {{
                outline: none;
                border-color: #3498db;
            }}

            .filter-clear-button {{
                position: absolute;
                right: 8px;
                background: none;
                border: none;
                font-size: 24px;
                color: #95a5a6;
                cursor: pointer;
                padding: 4px 8px;
                display: flex;
                align-items: center;
                justify-content: center;
                border-radius: 4px;
                transition: background 0.2s, color 0.2s;
                line-height: 1;
            }}

            .filter-clear-button:hover {{
                background: #f0f0f0;
                color: #2c3e50;
            }}

            .model-item {{
                background: white;
                padding: 16px 20px;
                margin-bottom: 12px;
                border-radius: 8px;
                border-left: 4px solid #3498db;
                transition: transform 0.2s, box-shadow 0.2s;
                cursor: pointer;
            }}

            .model-item:hover {{
                transform: translateX(4px);
                box-shadow: 0 4px 12px rgba(0,0,0,0.1);
            }}

            .model-name {{
                font-size: 18px;
                font-weight: 600;
                color: #2c3e50;
                margin-bottom: 8px;
            }}

            .modality-badges {{
                display: flex;
                gap: 6px;
                flex-wrap: wrap;
                margin-bottom: 10px;
            }}

            .modality-badge {{
                display: inline-block;
                padding: 4px 10px;
                font-size: 12px;
                font-weight: 500;
                border-radius: 12px;
                background: #ecf0f1;
                color: #2c3e50;
            }}

            .modality-badge.text {{ background: #3498db; color: white; }}
            .modality-badge.image {{ background: #9b59b6; color: white; }}
            .modality-badge.file {{ background: #e67e22; color: white; }}
            .modality-badge.embeddings {{ background: #1abc9c; color: white; }}
            .modality-badge.audio {{ background: #e74c3c; color: white; }}
            .modality-badge.video {{ background: #f1c40f; color: white; }}

            .modality-badge-outline {{
                display: inline-block;
                padding: 4px 10px;
                font-size: 12px;
                font-weight: 500;
                border-radius: 12px;
                background: transparent;
                border: 2px solid #bdc3c7;
                color: #7f8c8d;
            }}

            .modality-badge-outline.text {{ border-color: #3498db; color: #3498db; }}
            .modality-badge-outline.image {{ border-color: #9b59b6; color: #9b59b6; }}
            .modality-badge-outline.file {{ border-color: #e67e22; color: #e67e22; }}
            .modality-badge-outline.embeddings {{ border-color: #1abc9c; color: #1abc9c; }}
            .modality-badge-outline.audio {{ border-color: #e74c3c; color: #e74c3c; }}
            .modality-badge-outline.video {{ border-color: #f1c40f; color: #f1c40f; }}

            .modality-separator {{
                display: inline-flex;
                align-items: center;
                justify-content: center;
                padding: 4px 8px;
                font-size: 14px;
                font-weight: 700;
                border-radius: 10px;
                background: #ecf0f1;
                color: #7f8c8d;
                margin: 0 4px;
            }}

            .modality-filter-section {{
                background: white;
                padding: 16px;
                border-radius: 8px;
                border: 2px solid #e0e0e0;
                margin-bottom: 20px;
            }}

            .modality-filter-group {{
                margin-bottom: 16px;
            }}

            .modality-filter-group:last-child {{
                margin-bottom: 0;
            }}

            .modality-filter-label {{
                font-weight: 600;
                color: #34495e;
                margin-bottom: 8px;
                font-size: 14px;
                display: block;
            }}

            .modality-toggles {{
                display: flex;
                gap: 8px;
                flex-wrap: wrap;
            }}

            .modality-toggle-button {{
                padding: 8px 16px;
                font-size: 13px;
                font-weight: 600;
                border: 2px solid #e0e0e0;
                border-radius: 20px;
                background: #f8f9fa;
                color: #7f8c8d;
                cursor: pointer;
                transition: all 0.2s;
                user-select: none;
            }}

            .modality-toggle-button:hover {{
                border-color: #bdc3c7;
                transform: translateY(-1px);
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }}

            .modality-toggle-button.active {{
                color: white;
                border-color: transparent;
            }}

            .modality-toggle-button.active.text {{
                background: #3498db;
            }}

            .modality-toggle-button.active.image {{
                background: #9b59b6;
            }}

            .modality-toggle-button.active.file {{
                background: #e67e22;
            }}

            .modality-toggle-button.active.embeddings {{
                background: #1abc9c;
            }}

            .modality-toggle-button.active.audio {{
                background: #e74c3c;
            }}

            .modality-toggle-button.active.video {{
                background: #f1c40f;
            }}

            .sort-controls-container {{
                display: flex;
                align-items: center;
                gap: 12px;
                margin-bottom: 16px;
                flex-wrap: wrap;
            }}

            .sort-field-group {{
                display: flex;
                border-radius: 6px;
                overflow: hidden;
                border: 2px solid #e0e0e0;
            }}

            .sort-field-button {{
                padding: 8px 16px;
                font-size: 13px;
                font-weight: 600;
                background: #f8f9fa;
                color: #7f8c8d;
                border: none;
                border-right: 1px solid #e0e0e0;
                cursor: pointer;
                transition: all 0.2s;
                user-select: none;
                white-space: nowrap;
            }}

            .sort-field-button:last-child {{
                border-right: none;
            }}

            /* Mobile responsive layout for sort buttons */
            @media (max-width: 640px) {{
                .sort-field-group {{
                    flex-direction: column;
                    width: 100%;
                }}

                .sort-field-button {{
                    border-right: none;
                    border-bottom: 1px solid #e0e0e0;
                    padding: 12px 16px;
                    font-size: 14px;
                }}

                .sort-field-button:last-child {{
                    border-bottom: none;
                }}

                .sort-direction-button {{
                    width: 100%;
                    justify-content: center;
                    padding: 12px 16px;
                    font-size: 14px;
                }}
            }}

            .sort-field-button:hover {{
                background: #ecf0f1;
            }}

            .sort-field-button.active {{
                background: #3498db;
                color: white;
            }}

            .sort-direction-button {{
                padding: 8px 16px;
                font-size: 13px;
                font-weight: 600;
                border: 2px solid #e0e0e0;
                border-radius: 6px;
                background: #f8f9fa;
                color: #7f8c8d;
                cursor: pointer;
                transition: all 0.2s;
                user-select: none;
                display: flex;
                align-items: center;
                gap: 6px;
            }}

            .sort-direction-button:hover {{
                background: #ecf0f1;
                border-color: #bdc3c7;
            }}

            .model-metadata {{
                display: grid;
                grid-template-columns: auto 1fr;
                gap: 8px 12px;
                font-size: 13px;
                color: #555;
                margin-bottom: 10px;
            }}

            .model-list-container {{
                max-height: 600px;
                overflow-y: auto;
                overflow-x: hidden;
                scroll-behavior: smooth;
                padding-right: 4px;
            }}

            .model-list-container::-webkit-scrollbar {{
                width: 8px;
            }}

            .model-list-container::-webkit-scrollbar-track {{
                background: #ecf0f1;
                border-radius: 4px;
            }}

            .model-list-container::-webkit-scrollbar-thumb {{
                background: #bdc3c7;
                border-radius: 4px;
            }}

            .model-list-container::-webkit-scrollbar-thumb:hover {{
                background: #95a5a6;
            }}

            .metadata-label {{
                font-weight: 600;
                color: #7f8c8d;
            }}

            .metadata-value {{
                color: #2c3e50;
            }}

            .price-value {{
                font-family: 'Monaco', 'Consolas', monospace;
                color: #27ae60;
                font-weight: 600;
            }}

            .canonical-slug {{
                background: #f8f9fa;
                border: 1px solid #e0e0e0;
                border-radius: 4px;
                padding: 8px 10px;
                font-family: 'Monaco', 'Consolas', monospace;
                font-size: 13px;
                color: #34495e;
                user-select: all;
                cursor: text;
                overflow-x: auto;
            }}

            .retry-button {{
                padding: 10px 20px;
                background: #3498db;
                color: white;
                border: none;
                border-radius: 6px;
                cursor: pointer;
                font-size: 14px;
                font-weight: 600;
                transition: background 0.2s;
            }}

            .retry-button:hover {{
                background: #2980b9;
            }}

            @keyframes spin {{
                from {{ transform: rotate(0deg); }}
                to {{ transform: rotate(360deg); }}
            }}

            .modal-overlay {{
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.6);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 1000;
                padding: 20px;
                overflow-y: auto;
            }}

            .modal-content {{
                background: white;
                border-radius: 12px;
                max-width: 800px;
                width: 100%;
                max-height: 90vh;
                overflow-y: auto;
                position: relative;
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            }}

            .modal-header {{
                position: sticky;
                top: 0;
                background: white;
                padding: 20px 24px;
                border-bottom: 2px solid #e0e0e0;
                display: flex;
                justify-content: space-between;
                align-items: center;
                z-index: 10;
            }}

            .modal-title {{
                font-size: 22px;
                font-weight: 700;
                color: #2c3e50;
                margin: 0;
            }}

            .modal-close {{
                background: none;
                border: none;
                font-size: 28px;
                color: #95a5a6;
                cursor: pointer;
                padding: 0;
                width: 32px;
                height: 32px;
                display: flex;
                align-items: center;
                justify-content: center;
                border-radius: 4px;
                transition: background 0.2s, color 0.2s;
            }}

            .modal-close:hover {{
                background: #f0f0f0;
                color: #2c3e50;
            }}

            .modal-body {{
                padding: 24px;
            }}

            .modal-section {{
                margin-bottom: 24px;
            }}

            .modal-section:last-child {{
                margin-bottom: 0;
            }}

            .modal-section-title {{
                font-size: 16px;
                font-weight: 700;
                color: #2c3e50;
                margin-bottom: 12px;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                border-bottom: 2px solid #3498db;
                padding-bottom: 6px;
            }}

            .modal-grid {{
                display: grid;
                grid-template-columns: auto 1fr;
                gap: 10px 16px;
                font-size: 14px;
            }}

            .modal-label {{
                font-weight: 600;
                color: #7f8c8d;
            }}

            .modal-value {{
                color: #2c3e50;
            }}

            .modal-price-value {{
                font-family: 'Monaco', 'Consolas', monospace;
                color: #27ae60;
                font-weight: 600;
            }}

            .modal-parameters-list {{
                display: flex;
                flex-wrap: wrap;
                gap: 8px;
                margin-top: 8px;
            }}

            .modal-parameter-badge {{
                background: #ecf0f1;
                color: #2c3e50;
                padding: 6px 12px;
                border-radius: 6px;
                font-size: 13px;
                font-family: 'Monaco', 'Consolas', monospace;
            }}

            .copy-button {{
                background: #3498db;
                color: white;
                border: none;
                border-radius: 4px;
                padding: 6px 12px;
                font-size: 12px;
                font-weight: 600;
                cursor: pointer;
                transition: background 0.2s;
                margin-left: 8px;
            }}

            .copy-button:hover {{
                background: #2980b9;
            }}

            .copy-button:active {{
                background: #21618c;
            }}

            .copy-button.copied {{
                background: #27ae60;
            }}

            .canonical-slug-container {{
                display: flex;
                align-items: center;
                gap: 8px;
            }}
            "
        }
    }
}
